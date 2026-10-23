// NOTE: This module is mostly for analysing tags from songs.
// For example, both languages and release type don't have a dedicated Navidrome ID.
// They are only available from the tags field from /api/song (Navidrome API), so they belong here.
// If they did have an ID, they should go to the root of the analysis module and the api/most-played/ (Navalyze API) route

pub mod languages;
