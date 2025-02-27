use super::common_key_events;
use crate::app::{App, ArtistBlock, TrackTableContext};
use crate::event::Key;

fn handle_down_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist {
        match artist.artist_selected_block {
            ArtistBlock::TopTracks => {
                let next_index = common_key_events::on_down_press_handler(
                    &artist.top_tracks,
                    Some(artist.selected_top_track_index),
                );
                artist.selected_top_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_down_press_handler(
                    &artist.albums.items,
                    Some(artist.selected_album_index),
                );
                artist.selected_album_index = next_index;
            }
            ArtistBlock::RelatedArtists => {
                let next_index = common_key_events::on_down_press_handler(
                    &artist.related_artists,
                    Some(artist.selected_related_artist_index),
                );
                artist.selected_related_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_down_press_on_hovered_block(app: &mut App) {
    if let Some(artist) = &mut app.artist {
        match artist.artist_hovered_block {
            ArtistBlock::TopTracks => {
                artist.artist_hovered_block = ArtistBlock::Albums;
            }
            ArtistBlock::Albums => {
                artist.artist_hovered_block = ArtistBlock::RelatedArtists;
            }
            ArtistBlock::RelatedArtists => {
                artist.artist_hovered_block = ArtistBlock::TopTracks;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_up_press_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist {
        match artist.artist_selected_block {
            ArtistBlock::TopTracks => {
                let next_index = common_key_events::on_up_press_handler(
                    &artist.top_tracks,
                    Some(artist.selected_top_track_index),
                );
                artist.selected_top_track_index = next_index;
            }
            ArtistBlock::Albums => {
                let next_index = common_key_events::on_up_press_handler(
                    &artist.albums.items,
                    Some(artist.selected_album_index),
                );
                artist.selected_album_index = next_index;
            }
            ArtistBlock::RelatedArtists => {
                let next_index = common_key_events::on_up_press_handler(
                    &artist.related_artists,
                    Some(artist.selected_related_artist_index),
                );
                artist.selected_related_artist_index = next_index;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_up_press_on_hovered_block(app: &mut App) {
    if let Some(artist) = &mut app.artist {
        match artist.artist_hovered_block {
            ArtistBlock::TopTracks => {
                artist.artist_hovered_block = ArtistBlock::RelatedArtists;
            }
            ArtistBlock::Albums => {
                artist.artist_hovered_block = ArtistBlock::TopTracks;
            }
            ArtistBlock::RelatedArtists => {
                artist.artist_hovered_block = ArtistBlock::Albums;
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_enter_event_on_selected_block(app: &mut App) {
    if let Some(artist) = &mut app.artist.clone() {
        match artist.artist_selected_block {
            ArtistBlock::TopTracks => {
                let selected_index = artist.selected_top_track_index;
                let top_tracks = artist
                    .top_tracks
                    .iter()
                    .map(|track| track.uri.to_owned())
                    .collect();
                app.start_playback(None, Some(top_tracks), Some(selected_index));
            }
            ArtistBlock::Albums => {
                if let Some(selected_album) = artist
                    .albums
                    .items
                    .get(artist.selected_album_index)
                    .cloned()
                {
                    app.track_table.context = Some(TrackTableContext::AlbumSearch);
                    app.get_album_tracks(selected_album);
                }
            }
            ArtistBlock::RelatedArtists => {
                let selected_index = artist.selected_related_artist_index;
                let artist_id = &artist.related_artists[selected_index].id;
                let artist_name = &artist.related_artists[selected_index].name;
                app.get_artist(&artist_id, &artist_name);
            }
            ArtistBlock::Empty => {}
        }
    }
}

fn handle_enter_event_on_hovered_block(app: &mut App) {
    if let Some(artist) = &mut app.artist {
        match artist.artist_hovered_block {
            ArtistBlock::TopTracks => artist.artist_selected_block = ArtistBlock::TopTracks,
            ArtistBlock::Albums => artist.artist_selected_block = ArtistBlock::Albums,
            ArtistBlock::RelatedArtists => {
                artist.artist_selected_block = ArtistBlock::RelatedArtists
            }
            ArtistBlock::Empty => {}
        }
    }
}

pub fn handler(key: Key, app: &mut App) {
    if let Some(artist) = &mut app.artist {
        match key {
            Key::Esc => {
                artist.artist_selected_block = ArtistBlock::Empty;
            }
            k if common_key_events::down_event(k) => {
                if artist.artist_selected_block != ArtistBlock::Empty {
                    handle_down_press_on_selected_block(app);
                } else {
                    handle_down_press_on_hovered_block(app);
                }
            }
            k if common_key_events::up_event(k) => {
                if artist.artist_selected_block != ArtistBlock::Empty {
                    handle_up_press_on_selected_block(app);
                } else {
                    handle_up_press_on_hovered_block(app);
                }
            }
            k if common_key_events::left_event(k) => {
                artist.artist_selected_block = ArtistBlock::Empty;
                match artist.artist_hovered_block {
                    ArtistBlock::TopTracks => common_key_events::handle_left_event(app),
                    ArtistBlock::Albums => {
                        artist.artist_hovered_block = ArtistBlock::TopTracks;
                    }
                    ArtistBlock::RelatedArtists => {
                        artist.artist_hovered_block = ArtistBlock::Albums;
                    }
                    ArtistBlock::Empty => {}
                }
            }
            k if common_key_events::right_event(k) => {
                artist.artist_selected_block = ArtistBlock::Empty;
                handle_down_press_on_hovered_block(app);
            }
            Key::Enter => {
                if artist.artist_selected_block != ArtistBlock::Empty {
                    handle_enter_event_on_selected_block(app);
                } else {
                    handle_enter_event_on_hovered_block(app);
                }
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::ActiveBlock;

    #[test]
    fn on_esc() {
        let mut app = App::new();

        handler(Key::Esc, &mut app);

        let current_route = app.get_current_route();
        assert_eq!(current_route.active_block, ActiveBlock::Empty);
    }
}
