use uuid::Uuid;
use mongodb::bson::{Binary, Uuid as MongoUuid, spec::BinarySubtype, UuidRepresentation};
use crate::models::genre::{Genre, GenreDTO};

#[test]
fn correctly_maps_to_dto() {
    // given
    let name = String::from("Action");
    let uuid = Uuid::new_v4();
    let bin = Binary {
        subtype: BinarySubtype::UuidOld,
        bytes: uuid.as_bytes().to_vec(),
    };
    let genre = Genre::new(Some(bin), String::from(&name));
    let expected_dto = GenreDTO::new(Some(uuid), name);
    // when
    let actual_dto = genre.to_dto().unwrap();
    // then
    assert_eq!(expected_dto, actual_dto);
}

#[test]
fn correctly_maps_to_genre() {
    // given
    let name = String::from("Action");
    let uuid = Uuid::new_v4();
    let bin = Binary::from_uuid_with_representation(MongoUuid::from_bytes(*uuid.as_bytes()), UuidRepresentation::JavaLegacy);
    let genre_dto = GenreDTO::new(Some(uuid), String::from(&name));
    let expected_genre = Genre::new(Some(bin), String::from(&name));
    // when
    let actual_genre = genre_dto.to_genre().unwrap();
    // then
    assert_eq!(expected_genre, actual_genre);
}