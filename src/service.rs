use salvo::{Router, Service};

use crate::{
    default_handler::default_response_middleware,
    interfaces::{
        delete_student, get_student, get_student_list, put_student,
    },
};

pub fn app_service() -> Service {
    Service::new(
        Router::new().push(
            Router::with_path("student")
                .get(get_student)
                .delete(delete_student)
                .put(put_student)
                .push(Router::with_path("list").get(get_student_list)),
        ),
    ).hoop(default_response_middleware)
}
