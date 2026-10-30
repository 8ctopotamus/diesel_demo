use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
  use self::schema::posts::dsl::*;

  let connection = &mut establish_connection();
  let results = posts.filter(published.eq(true))
}