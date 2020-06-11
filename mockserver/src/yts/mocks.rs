use super::data::MovieResponse;
use super::data::YtsResponse;

pub fn mockYtsMovies() -> YtsResponse {
  let armageddon = MovieResponse {
    id: String::from("Armageddon"),
    name: String::from("Armageddon"),
    imdb_rate: String::from("8"),
    url: String::from("http://armageddon.com"),
  };

  let titanic = MovieResponse {
    id: String::from("titanicss"),
    name: String::from("Titanic"),
    imdb_rate: String::from("8"),
    url: String::from("http://titanic.com"),
  };

  let movies = vec![armageddon, titanic];

  YtsResponse { movies: movies }
}
