pub async fn create(req: HttpRequest) -> impl Responder {
    // obtain the title of the to-do item from the request
    let title = req.match_info().get("title").unwrap();
    // establish a database connection
    let mut connection = establish_connection();
    // make a database call to table
    let items = to_do::table.filter(to_do::columns::title.eq(title));
    // check the item being created exists in the database, if not, create an item and insert it into the database
    if let Ok(0) = items.count().get_result(&mut connection) {
        let new_item = NewItem::new(title);
        diesel::insert_into(to_do::table)
            .values(new_item)
            .execute(&mut connection)
            .unwrap();
    }
    return_state()
}

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title = &to_do_item.title;

    let mut connection = establish_connection();

    let items = to_do::table.filter(to_do::columns::title.eq(title));

    diesel::delete(items).execute(&mut connection).unwrap();

    HttpResponse::Ok().json(return_state())
}

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title = &to_do_item.title;
    let mut connection = establish_connection();
    let result = to_do::table.filter(to_do::columns::title.eq(title));
    diesel::update(result)
        .set(to_do::columns::status.eq("done"))
        .execute(&mut connection)
        .unwrap();
    HttpResponse::Ok().json(return_state())
}
