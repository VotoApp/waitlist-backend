use lambda_http::{Body, Error as LambdaError, Response};
use tracing::debug;

pub fn get_invalid_input_response() -> Result<Response<Body>, LambdaError> {
    debug!("Creating an invalid input response");
    let response = Response::builder()
        .status(400)
        .body(Body::Text(String::new()))?;
    return Ok(response);
}

pub fn get_bad_gateway_response() -> Result<Response<Body>, LambdaError> {
    debug!("Creating an bad gateway response");
    let response = Response::builder()
        .status(502)
        .body(Body::Text(String::new()))?;
    return Ok(response);
}

pub fn get_resource_conflict_response() -> Result<Response<Body>, LambdaError> {
    debug!("Creating a resource conflict response");
    let response = Response::builder()
        .status(409)
        .body(Body::Text(String::new()))?;
    return Ok(response);
}

pub fn get_all_good_response() -> Result<Response<Body>, LambdaError> {
    debug!("Creating an all good response");
    let response = Response::builder()
        .status(200)
        .body(Body::Text(String::new()))?;
    return Ok(response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_response_snapshot() {
        // Arrange
        let expected =
            "Ok(Response { status: 400, version: HTTP/1.1, headers: {}, body: Text(\"\") })";

        // Act
        let actual = format!("{:?}", get_invalid_input_response());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bad_gateway_response_snapshot() {
        // Arrange
        let expected =
            "Ok(Response { status: 502, version: HTTP/1.1, headers: {}, body: Text(\"\") })";

        // Act
        let actual = format!("{:?}", get_bad_gateway_response());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_resource_conflict_response_snapshot() {
        // Arrange
        let expected =
            "Ok(Response { status: 409, version: HTTP/1.1, headers: {}, body: Text(\"\") })";

        // Act
        let actual = format!("{:?}", get_resource_conflict_response());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all_good_response_snapshot() {
        // Arrange
        let expected =
            "Ok(Response { status: 200, version: HTTP/1.1, headers: {}, body: Text(\"\") })";

        // Act
        let actual = format!("{:?}", get_all_good_response());

        // Assert
        assert_eq!(actual, expected);
    }
}
