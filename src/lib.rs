use debug::tee;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use primitives::{
    read_f32, read_f64, read_i16, read_i32, read_i64, read_i8, read_lps, read_u16, read_u32,
    read_u64, read_u8,
};
use std::collections::HashMap;
use std::io;
use value::Value;

mod debug;
mod primitives;
pub mod value;

trait FromStream {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self;
}

// The following makes all `FromPrimitive` enums readable directly from stream.
impl<T: FromPrimitive> FromStream for T {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        let byte = read_u8(stream);
        match FromPrimitive::from_u8(byte) {
            Some(enum_val) => enum_val,
            None => panic!("Unexpected enum value {byte:?}"),
        }
    }
}

#[derive(Debug, FromPrimitive)]
enum RecordType {
    SerializationHeader = 0,
    ClassWithId = 1,
    SystemClassWithMembers = 2,
    ClassWithMembers = 3,
    SystemClassWithMembersAndTypes = 4,
    ClassWithMembersAndTypes = 5,
    BinaryObjectString = 6,
    BinaryArray = 7,
    MemberPrimitiveTyped = 8,
    MemberReference = 9,
    ObjectNull = 10,
    MessageEnd = 11,
    BinaryLibrary = 12,
    ObjectNullMultiple256 = 13,
    ObjectNullMultiple = 14,
    ArraySinglePrimitive = 15,
    ArraySingleObject = 16,
    ArraySingleString = 17,
    MethodCall = 21,
    MethodReturn = 22,
}

#[derive(Debug, FromPrimitive, Clone)]
enum BinaryType {
    Primitive = 0,
    String = 1,
    Object = 2,
    SystemClass = 3,
    Class = 4,
    ObjectArray = 5,
    StringArray = 6,
    PrimitiveArray = 7,
    Record, // Additional field, not in spec.
}

#[derive(Debug, FromPrimitive, Clone)]
enum PrimitiveType {
    Boolean = 1,
    Byte = 2,
    Char = 3,
    Decimal = 5,
    Double = 6,
    Int16 = 7,
    Int32 = 8,
    Int64 = 9,
    SByte = 10,
    Single = 11,
    TimeSpan = 12,
    DateTime = 13,
    UInt16 = 14,
    UInt32 = 15,
    UInt64 = 16,
    Null = 17,
    String = 18,
}

impl PrimitiveType {
    fn read<R: io::Read>(&self, stream: &mut R) -> Value {
        match self {
            PrimitiveType::Boolean => Value::Bool(read_u8(stream) != 0),
            // case PrimitiveType.Char:
            // case PrimitiveType.Decimal:
            // case PrimitiveType.TimeSpan :
            // case PrimitiveType.DateTime:
            PrimitiveType::SByte => Value::I8(read_i8(stream)),
            PrimitiveType::Int16 => Value::I32(read_i16(stream) as i32),
            PrimitiveType::Int32 => Value::I32(read_i32(stream)),
            PrimitiveType::Int64 => Value::I64(read_i64(stream)),
            PrimitiveType::Byte => Value::U8(read_u8(stream)),
            PrimitiveType::UInt16 => Value::U32(read_u16(stream) as u32),
            PrimitiveType::UInt32 => Value::U32(read_u32(stream)),
            PrimitiveType::UInt64 => Value::U64(read_u64(stream)),
            PrimitiveType::Single => Value::F32(read_f32(stream)),
            PrimitiveType::Double => Value::F64(read_f64(stream)),
            PrimitiveType::Null => Value::Null,
            PrimitiveType::String => Value::String(read_lps(stream)),
            _ => panic!("Cannot deserialize {self:?} yet"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, FromPrimitive)]
enum BinaryArrayType {
    /// A single-dimensional Array.
    Single = 0,
    /// An Array whose elements are Arrays. The elements of a jagged Array can be of different dimensions and sizes.
    Jagged = 1,
    /// A multi-dimensional rectangular Array.
    Rectangular = 2,
    /// A single-dimensional offset.
    SingleOffset = 3,
    /// A jagged Array where the lower bound index is greater than 0.
    JaggedOffset = 4,
    /// Multi-dimensional Arrays where the lower bound index of at least one of the dimensions is greater than 0.
    RectangularOffset = 5,
}

struct ClassInfo {
    id: i32,
    name: String,
    field_names: Vec<String>,
}

impl FromStream for ClassInfo {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        let id = read_i32(stream);
        let name = read_lps(stream);
        let member_count = read_i32(stream);
        let member_names = (0..member_count).map(|_| read_lps(stream)).collect();
        Self {
            id,
            name,
            field_names: member_names,
        }
    }
}

#[derive(Debug, Clone)]
struct ClassTypeInfo {
    _name: String,
    _library_id: i32,
}

impl FromStream for ClassTypeInfo {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        Self {
            _name: read_lps(stream),
            _library_id: read_i32(stream),
        }
    }
}

#[derive(Debug, Clone)]
enum AdditionalInfos {
    Nothing,
    PrimitiveType(PrimitiveType),
    ClassName(String),
    Class(ClassTypeInfo),
}

impl AdditionalInfos {
    fn from_stream<R: io::Read>(stream: &mut R, binary_type: BinaryType) -> Self {
        match binary_type {
            BinaryType::Primitive | BinaryType::PrimitiveArray => {
                AdditionalInfos::PrimitiveType(PrimitiveType::from_stream(stream))
            }
            BinaryType::SystemClass => AdditionalInfos::ClassName(read_lps(stream)),
            BinaryType::Class => AdditionalInfos::Class(ClassTypeInfo::from_stream(stream)),
            _ => AdditionalInfos::Nothing,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassField(String, BinaryType, AdditionalInfos);
#[derive(Debug, Clone)]
pub struct Class(String, Vec<ClassField>);

struct DecoderState<'a, R: io::Read> {
    stream: &'a mut R,

    root_id: Option<i32>,
    header_id: Option<i32>,

    libraries: HashMap<i32, String>,
    classes: HashMap<i32, Class>,
    values: HashMap<i32, Value>,

    // This is a bit of a hack. NRBF encodes sequences of nulls as either NullMultiple or
    // NullMultiple256. The problem is that a record can "contain" multiple values in sequence. To
    // unpack this, we use `null_count`, which is used to emit Null values instead of reading more
    // records, when a null multiple has been encountered.
    null_count: usize,
}

impl<'a, R: io::Read> DecoderState<'a, R> {
    fn new(stream: &'a mut R) -> Self {
        DecoderState {
            stream,
            root_id: Default::default(),
            header_id: Default::default(),
            libraries: Default::default(),
            classes: Default::default(),
            values: Default::default(),

            null_count: Default::default(),
        }
    }

    fn parse_class_member(&mut self, class_field: &ClassField) -> (String, Value) {
        let ClassField(field_name, binary_type, additional_infos) = class_field;
        let value = match (binary_type, additional_infos) {
            (BinaryType::Record, AdditionalInfos::Nothing) => self.next_value_record(),
            (BinaryType::Primitive, AdditionalInfos::PrimitiveType(primitive_type)) => {
                primitive_type.read(self.stream)
            }
            (BinaryType::String, AdditionalInfos::Nothing) => self.next_value_record(),
            (BinaryType::SystemClass, AdditionalInfos::ClassName(_system_class_name)) => {
                self.next_value_record()
            }
            (BinaryType::Class, AdditionalInfos::Class(_)) => self.next_value_record(),
            (BinaryType::PrimitiveArray, AdditionalInfos::PrimitiveType(_primitive_type)) => {
                self.next_value_record()
            }
            _ => panic!("No parser for {binary_type:?}/{additional_infos:?} implemented"),
        };

        (field_name.clone(), value)
    }

    fn parse_object(&mut self, class_id: i32) -> Value {
        let Class(class_name, fields) = self
            .classes
            .get(&class_id)
            .expect(&format!("Class {class_id} is not yet defined"))
            .clone();
        let members = fields
            .iter()
            .map(|class_field| self.parse_class_member(class_field))
            .collect::<HashMap<_, _>>();
        Value::Object(class_name.clone(), members)
    }

    fn next_value_record(&mut self) -> Value {
        if self.null_count > 0 {
            self.null_count -= 1;
            return Value::Null;
        }

        match RecordType::from_stream(self.stream) {
            // Non-value records.
            RecordType::SerializationHeader => {
                self.root_id = Some(read_i32(self.stream));
                self.header_id = Some(read_i32(self.stream));
                let major_version = read_i32(self.stream);
                assert_eq!(major_version, 1, "Major version must be 1");
                let minor_version = read_i32(self.stream);
                assert_eq!(minor_version, 0, "Minor version must be 0");
                Value::Bottom
            }
            RecordType::BinaryLibrary => {
                let id = read_i32(self.stream);
                let name = read_lps(self.stream);
                self.libraries.insert(id, name);
                Value::Bottom
            }
            RecordType::MessageEnd => Value::Bottom,
            // Classes.
            RecordType::ClassWithId => {
                // New instance of a class, creates new object id, reuses previous class id.
                let id = read_i32(self.stream);

                // An INT32 value (as specified in [MS-DTYP] section 2.2.22) that references one
                // of the other Class records by its ObjectId. A SystemClassWithMembers,
                // SystemClassWithMembersAndTypes, ClassWithMembers, or ClassWithMembersAndTypes
                // record with the value of this field in its ObjectId field MUST appear earlier
                // in the serialization stream.
                let class_id = read_i32(self.stream);
                let object = self.parse_object(class_id);

                self.values.insert(id, object);
                Value::Reference(id)
            }
            RecordType::ClassWithMembers => {
                // New instance of a NEW class, TODO has no object id, creates class id.
                // Holds member names, types not needed, they are records.
                let ClassInfo {
                    id,
                    name: class_name,
                    field_names,
                } = ClassInfo::from_stream(self.stream);
                let _library_id = read_i32(self.stream);

                let class_fields = field_names
                    .iter()
                    .map(|name| {
                        ClassField(name.clone(), BinaryType::Record, AdditionalInfos::Nothing)
                    })
                    .collect();

                let class = Class(class_name, class_fields);
                self.classes.insert(id, class);

                let object = tee(self.parse_object(id));

                self.values.insert(id, object);
                Value::Reference(id)
            }
            RecordType::ClassWithMembersAndTypes => {
                // New instance of a NEW class.
                let ClassInfo {
                    id,
                    name: class_name,
                    field_names,
                } = ClassInfo::from_stream(self.stream);
                let binary_types = field_names
                    .iter()
                    .map(|_| BinaryType::from_stream(self.stream))
                    .collect::<Vec<_>>();
                let additional_infos = binary_types
                    .iter()
                    .cloned()
                    .map(|binary_type| AdditionalInfos::from_stream(self.stream, binary_type))
                    .collect::<Vec<_>>();
                let _library_id = read_i32(self.stream);

                let class_fields = field_names
                    .iter()
                    .zip(binary_types.into_iter())
                    .zip(additional_infos.into_iter())
                    .map(|((name, binary_type), additional_infos)| {
                        ClassField(name.clone(), binary_type, additional_infos)
                    })
                    .collect();

                let class = Class(class_name, class_fields);
                self.classes.insert(id, class);

                let object = tee(self.parse_object(id));

                self.values.insert(id, object);
                Value::Reference(id)
            }
            RecordType::SystemClassWithMembersAndTypes => {
                // New instance of a NEW system (std) class, TODO has no object id, creates class id.
                let ClassInfo {
                    id,
                    name: class_name,
                    field_names,
                } = ClassInfo::from_stream(self.stream);
                let binary_types = field_names
                    .iter()
                    .map(|_| BinaryType::from_stream(self.stream))
                    .collect::<Vec<_>>();
                let additional_infos = binary_types
                    .iter()
                    .cloned()
                    .map(|binary_type| AdditionalInfos::from_stream(self.stream, binary_type))
                    .collect::<Vec<_>>();

                let class_fields = field_names
                    .iter()
                    .zip(binary_types.into_iter())
                    .zip(additional_infos.into_iter())
                    .map(|((name, binary_type), additional_infos)| {
                        ClassField(name.clone(), binary_type, additional_infos)
                    })
                    .collect();

                let class = Class(class_name, class_fields);
                self.classes.insert(id, class);

                let object = tee(self.parse_object(id));

                self.values.insert(id, object);
                Value::Reference(id)
            }
            // Arrays.
            RecordType::BinaryArray => {
                let object_id = read_i32(self.stream);
                let array_type = BinaryArrayType::from_stream(self.stream);
                let rank = read_i32(self.stream);
                let lengths = (0..rank)
                    .map(|_| read_i32(self.stream) as usize)
                    .collect::<Vec<_>>();
                let lower_bounds = if array_type == BinaryArrayType::SingleOffset
                    || array_type == BinaryArrayType::JaggedOffset
                    || array_type == BinaryArrayType::RectangularOffset
                {
                    (0..rank).map(|_| read_i32(self.stream) as usize).collect()
                } else {
                    vec![0; rank.try_into().unwrap()]
                };
                let item_type = BinaryType::from_stream(self.stream);
                let _additional_info = AdditionalInfos::from_stream(self.stream, item_type);

                let size = lengths.iter().fold(1, |x, y| x * y);
                let values = (0..size).map(|_| self.next_value_record()).collect();
                self.values
                    .insert(object_id, Value::Array(lengths, lower_bounds, values));
                Value::Reference(object_id)
            }
            RecordType::ArraySinglePrimitive => {
                let object_id = read_i32(self.stream);
                let length = read_i32(self.stream) as usize;
                let primitive = PrimitiveType::from_stream(self.stream);
                let values = (0..length).map(|_| primitive.read(self.stream)).collect();
                self.values
                    .insert(object_id, Value::Array(vec![length], vec![0], values));
                Value::Reference(object_id)
            }
            RecordType::BinaryObjectString => {
                let id = read_i32(self.stream);
                let value = read_lps(self.stream);
                self.values.insert(id, tee(Value::String(value)));
                Value::Reference(id)
            }
            // Null sequences.
            RecordType::ObjectNull => Value::Null,
            RecordType::ObjectNullMultiple256 => {
                assert_eq!(self.null_count, 0);
                self.null_count = read_u8(self.stream) as usize;
                self.next_value_record()
            }
            RecordType::ObjectNullMultiple => {
                assert_eq!(self.null_count, 0);
                self.null_count = read_i32(self.stream) as usize;
                self.next_value_record()
            }
            // Other.
            // RecordType::MemberPrimitiveTyped            => Record::MemberPrimitiveTyped(MemberPrimitiveTyped::from_stream(stream)),
            RecordType::MemberReference => Value::Reference(read_i32(self.stream)),
            // self.values
            //     .remove(&id)
            //     .expect("Reference was either already used or never defined.")
            other => panic!("Unhandled record type: {other:?}"),
        }
    }

    fn resolve_references(&mut self, v: Value) -> Value {
        match v {
            Value::Object(class, members) => Value::Object(
                class,
                members
                    .into_iter()
                    .map(|(k, v)| (k, self.resolve_references(v)))
                    .collect(),
            ),
            Value::Array(a, b, values) => Value::Array(
                a,
                b,
                values
                    .into_iter()
                    .map(|v| self.resolve_references(v))
                    .collect(),
            ),
            Value::Reference(id) => loop {
                if let Some(v) = self.values.get(&id) {
                    return self.resolve_references(v.clone());
                }
                self.next_value_record();
            },
            other => other,
        }
    }
}

pub fn parse_nrbf<R: io::Read>(stream: &mut R) -> Value {
    let mut decoder = DecoderState::new(stream);
    while decoder.root_id.is_none() {
        decoder.next_value_record();
    }

    let root_id = decoder.root_id.unwrap();
    let root = decoder.resolve_references(Value::Reference(root_id));
    let end = decoder.next_value_record();
    assert_eq!(end, Value::Bottom);

    root
}
