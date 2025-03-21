///
/// @package heos-dial
///
/// @file HEOS lib
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

#[derive(Default, Clone, PartialEq, Debug)]
pub enum HeosMediaSourceType {
    #[default]
    Player,
    Station,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct HeosMedia {
    pub source_type: HeosMediaSourceType,
    pub artist_title: String,
    pub song_title: String,
    pub album_title: String,
    pub image_url: String,
}