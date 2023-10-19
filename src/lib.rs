use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::io;

trait FromStream {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self;
}

impl<T: FromPrimitive> FromStream for T {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        let mut bytes = [0u8; 1];
        match stream.read_exact(&mut bytes) {
            Ok(()) => (),
            Err(error) => panic!("Cannot read from stream: {error}"),
        };
        match FromPrimitive::from_u8(bytes[0]) {
            Some(enum_val) => enum_val,
            None => panic!("Unexpected enum value {}", bytes[0]),
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

pub struct SerializationHeader {}
impl FromStream for SerializationHeader {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ClassWithId {}
impl FromStream for ClassWithId {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ClassWithMembers {}
impl FromStream for ClassWithMembers {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct SystemClassWithMembersAndTypes {}
impl FromStream for SystemClassWithMembersAndTypes {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ClassWithMembersAndTypes {}
impl FromStream for ClassWithMembersAndTypes {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct BinaryObjectString {}
impl FromStream for BinaryObjectString {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct BinaryArray {}
impl FromStream for BinaryArray {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct MemberReference {}
impl FromStream for MemberReference {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ObjectNull {}
impl FromStream for ObjectNull {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct BinaryLibrary {}
impl FromStream for BinaryLibrary {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ObjectNullMultiple256 {}
impl FromStream for ObjectNullMultiple256 {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ObjectNullMultiple {}
impl FromStream for ObjectNullMultiple {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}
pub struct ArraySinglePrimitive {}
impl FromStream for ArraySinglePrimitive {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        todo!()
    }
}

enum Record {
    SerializationHeader(SerializationHeader),
    ClassWithId(ClassWithId),
    // RecordType.SystemClassWithMembers: None,
    ClassWithMembers(ClassWithMembers),
    SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes),
    ClassWithMembersAndTypes(ClassWithMembersAndTypes),
    BinaryObjectString(BinaryObjectString),
    BinaryArray(BinaryArray),
    // RecordType.MemberPrimitiveTyped: None,
    MemberReference(MemberReference),
    ObjectNull(ObjectNull),
    // RecordType.MessageEnd: None,
    BinaryLibrary(BinaryLibrary),
    ObjectNullMultiple256(ObjectNullMultiple256),
    ObjectNullMultiple(ObjectNullMultiple),
    ArraySinglePrimitive(ArraySinglePrimitive),
    // RecordType.ArraySingleObject: None,
    // RecordType.ArraySingleString: None,
    // RecordType.MethodCall: None,
    // RecordType.MethodReturn: None,
}

impl FromStream for Record {
    fn from_stream<R: io::Read>(stream: &mut R) -> Self {
        match RecordType::from_stream(stream) {
            RecordType::SerializationHeader => {
                Record::SerializationHeader(SerializationHeader::from_stream(stream))
            }
            RecordType::ClassWithId => Record::ClassWithId(ClassWithId::from_stream(stream)),
            // RecordType::SystemClassWithMembers          => Record::SystemClassWithMembers(SystemClassWithMembers::from_stream(stream)),
            RecordType::ClassWithMembers => {
                Record::ClassWithMembers(ClassWithMembers::from_stream(stream))
            }
            RecordType::SystemClassWithMembersAndTypes => Record::SystemClassWithMembersAndTypes(
                SystemClassWithMembersAndTypes::from_stream(stream),
            ),
            RecordType::ClassWithMembersAndTypes => {
                Record::ClassWithMembersAndTypes(ClassWithMembersAndTypes::from_stream(stream))
            }
            RecordType::BinaryObjectString => {
                Record::BinaryObjectString(BinaryObjectString::from_stream(stream))
            }
            RecordType::BinaryArray => Record::BinaryArray(BinaryArray::from_stream(stream)),
            // RecordType::MemberPrimitiveTyped            => Record::MemberPrimitiveTyped(MemberPrimitiveTyped::from_stream(stream)),
            RecordType::MemberReference => {
                Record::MemberReference(MemberReference::from_stream(stream))
            }
            RecordType::ObjectNull => Record::ObjectNull(ObjectNull::from_stream(stream)),
            // RecordType::MessageEnd                      => Record::MessageEnd(MessageEnd::from_stream(stream)),
            RecordType::BinaryLibrary => Record::BinaryLibrary(BinaryLibrary::from_stream(stream)),
            RecordType::ObjectNullMultiple256 => {
                Record::ObjectNullMultiple256(ObjectNullMultiple256::from_stream(stream))
            }
            RecordType::ObjectNullMultiple => {
                Record::ObjectNullMultiple(ObjectNullMultiple::from_stream(stream))
            }
            RecordType::ArraySinglePrimitive => {
                Record::ArraySinglePrimitive(ArraySinglePrimitive::from_stream(stream))
            }
            // RecordType::ArraySingleObject               => Record::ArraySingleObject(ArraySingleObject::from_stream(stream)),
            // RecordType::ArraySingleString               => Record::ArraySingleString(ArraySingleString::from_stream(stream)),
            // RecordType::MethodCall                      => Record::MethodCall(MethodCall::from_stream(stream)),
            // RecordType::MethodReturn                    => Record::MethodReturn(MethodReturn::from_stream(stream)),
            other => panic!("Unhandled record type: {other:?}")
        }
    }
}

pub struct Class {}

pub fn parse<R: io::Read>(stream: &mut R) -> (HashMap<String, Class>, HashMap<String, Record>) {
    let mut classes = HashMap::new();
    let mut records = HashMap::new();

    loop {


    }
}

// #[cfg(test)]
// mod tests {
// }
