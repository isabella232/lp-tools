table! {
    album_names (id) {
        id -> Int4,
        album_id -> Int4,
        name -> Varchar,
        locale -> Varchar,
        is_default -> Bool,
        is_original -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    albums (id) {
        id -> Int4,
        artist_credit_id -> Int4,
        kind -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    artist_credit_names (id) {
        id -> Int4,
        artist_id -> Int4,
        artist_credit_id -> Int4,
        position -> Int2,
        name -> Varchar,
        locale -> Varchar,
        is_default -> Bool,
        is_original -> Bool,
        separator -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    artist_credits (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    artist_names (id) {
        id -> Int4,
        artist_id -> Int4,
        name -> Varchar,
        locale -> Varchar,
        is_default -> Bool,
        is_original -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    artist_urls (id) {
        id -> Int4,
        artist_id -> Int4,
        url -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    artists (id) {
        id -> Int4,
        kind -> Int4,
        country -> Varchar,
        disambiguation -> Nullable<Varchar>,
        started_on_year -> Nullable<Int2>,
        started_on_month -> Nullable<Int2>,
        started_on_day -> Nullable<Int2>,
        ended_on_year -> Nullable<Int2>,
        ended_on_month -> Nullable<Int2>,
        ended_on_day -> Nullable<Int2>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    contributions (id) {
        id -> Int4,
        artist_credit_id -> Int4,
        song_id -> Int4,
        kind -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    media (id) {
        id -> Int4,
        release_id -> Int4,
        kind -> Int4,
        position -> Int2,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    memberships (id) {
        id -> Int4,
        group_id -> Int4,
        artist_credit_id -> Int4,
        started_on_year -> Nullable<Int2>,
        started_on_month -> Nullable<Int2>,
        started_on_day -> Nullable<Int2>,
        ended_on_year -> Nullable<Int2>,
        ended_on_month -> Nullable<Int2>,
        ended_on_day -> Nullable<Int2>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    release_urls (id) {
        id -> Int4,
        release_id -> Int4,
        url -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    releases (id) {
        id -> Int4,
        album_id -> Int4,
        released_on -> Date,
        country -> Nullable<Varchar>,
        catalog_number -> Nullable<Varchar>,
        disambiguation -> Nullable<Varchar>,
        artwork_data -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    song_names (id) {
        id -> Int4,
        song_id -> Int4,
        name -> Varchar,
        locale -> Varchar,
        is_default -> Bool,
        is_original -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    song_urls (id) {
        id -> Int4,
        song_id -> Int4,
        url -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    songs (id) {
        id -> Int4,
        artist_credit_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    track_names (id) {
        id -> Int4,
        track_id -> Int4,
        name -> Varchar,
        locale -> Varchar,
        is_default -> Bool,
        is_original -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tracks (id) {
        id -> Int4,
        medium_id -> Int4,
        artist_credit_id -> Int4,
        song_id -> Int4,
        position -> Int2,
        duration -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(albums -> artist_credits (artist_credit_id));
joinable!(artist_credit_names -> artist_credits (artist_credit_id));
joinable!(artist_credit_names -> artists (artist_id));
joinable!(artist_names -> artists (artist_id));
joinable!(artist_urls -> artists (artist_id));
joinable!(contributions -> artist_credits (artist_credit_id));
joinable!(contributions -> songs (song_id));
joinable!(media -> releases (release_id));
joinable!(memberships -> artist_credits (artist_credit_id));
joinable!(memberships -> artists (group_id));
joinable!(release_urls -> releases (release_id));
joinable!(song_names -> songs (song_id));
joinable!(song_urls -> songs (song_id));
joinable!(songs -> artist_credits (artist_credit_id));
joinable!(track_names -> tracks (track_id));
joinable!(tracks -> artist_credits (artist_credit_id));
joinable!(tracks -> media (medium_id));
joinable!(tracks -> songs (song_id));

allow_tables_to_appear_in_same_query!(
    album_names,
    albums,
    artist_credit_names,
    artist_credits,
    artist_names,
    artist_urls,
    artists,
    contributions,
    media,
    memberships,
    release_urls,
    releases,
    song_names,
    song_urls,
    songs,
    track_names,
    tracks,
);
