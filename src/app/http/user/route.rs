// pub fn build_routes(router: Router) -> Router {
//     return router.route("/health-check", get(|| async {StatusCode::OK}))
//     .route("/users", post(create_user));
// }

// PREPARAR O CONTAINER PARA CONTER AS DEPENDENCIAS DE INFRA E ASSIM PERMITIR A CONSTRUÇÃO DOS REPOSITORIES E USECASES