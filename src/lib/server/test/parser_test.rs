#[cfg(test)]
mod parser_test {
    use super::super::*;

    #[test]
    fn create_parser_with_simple_frame_return_error() {
        let parser = Parser::new(Frame::Simple("Hello".into()));
        assert!(parser.is_err());
    }

    #[test]
    fn create_parser_with_error_frame_return_error() {
        let parser = Parser::new(Frame::Error("Error".into()));
        assert!(parser.is_err());
    }

    #[test]
    fn create_parser_with_integer_frame_return_error() {
        let parser = Parser::new(Frame::Integer(42));
        assert!(parser.is_err());
    }

    #[test]
    fn create_parser_with_bulk_frame_return_error() {
        let parser = Parser::new(Frame::Bulk(Bytes::from("Hello")));
        assert!(parser.is_err());
    }

    #[test]
    fn create_parser_with_array_frame_return_ok() {
        let parser = Parser::new(Frame::Array(vec![
            Frame::Bulk(Bytes::from("Hello")),
            Frame::Bulk(Bytes::from("Hi")),
        ]));
        assert!(parser.is_ok());
    }

    #[test]
    fn next_string_returns_string_if_simple_frame() {
        let mut parser = Parser::new(Frame::Array(vec![
            Frame::Simple("Hello".into()),
            Frame::Bulk(Bytes::from("Hi")),
        ]))
        .unwrap();
        assert_eq!(parser.next_string().unwrap(), "Hello");
    }

    #[test]
    fn next_string_returns_string_if_bulk_frame_can_be_converted_into_string() {
        let mut parser =
            Parser::new(Frame::Array(vec![Frame::Bulk(Bytes::from("Hello"))])).unwrap();
        assert_eq!(parser.next_string().unwrap(), "Hello");
    }

    #[test]
    fn next_string_returns_error_if_bulk_frame_can_not_be_converted_into_string() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Bulk(Bytes::from(vec![
            0xff, 0x02, 0x03,
        ]))]))
        .unwrap();
        assert!(parser.next_string().is_err());
    }

    #[test]
    fn next_string_returns_error_if_not_simple_or_bulk_frame() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Error("Error".into())])).unwrap();
        assert!(parser.next_string().is_err());
    }

    #[test]
    fn next_int_returns_u64_if_integer_frame() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Integer(42)])).unwrap();
        assert_eq!(parser.next_int().unwrap(), 42);
    }

    #[test]
    fn next_int_returns_u64_if_simple_frame_can_be_converted_into_u64() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Simple("42".into())])).unwrap();
        assert_eq!(parser.next_int().unwrap(), 42);
    }

    #[test]
    fn next_int_returns_error_if_simple_frame_can_not_be_converted_into_u64() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Simple("Hello".into())])).unwrap();
        assert!(parser.next_int().is_err());
    }

    #[test]
    fn next_int_returns_u64_if_bulk_frame_can_be_converted_into_u64() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Bulk(Bytes::from("42"))])).unwrap();
        assert_eq!(parser.next_int().unwrap(), 42);
    }

    #[test]
    fn next_int_returns_error_if_bulk_frame_can_not_be_converted_into_u64() {
        let mut parser =
            Parser::new(Frame::Array(vec![Frame::Bulk(Bytes::from("Hello"))])).unwrap();
        assert!(parser.next_int().is_err());
    }

    #[test]
    fn next_int_returns_error_if_not_integer_simple_or_bulk_frame() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Null])).unwrap();
        assert!(parser.next_int().is_err());
    }

    #[test]
    fn next_bytes_returns_bytes_if_simple_frame() {
        let mut parser =
            Parser::new(Frame::Array(vec![Frame::Simple("Hello".into())]).into()).unwrap();
        assert_eq!(parser.next_bytes().unwrap(), Bytes::from("Hello"));
    }

    #[test]
    fn next_bytes_returns_bytes_if_bulk_frame() {
        let mut parser =
            Parser::new(Frame::Array(vec![Frame::Bulk(Bytes::from("Hello"))])).unwrap();
        assert_eq!(parser.next_bytes().unwrap(), Bytes::from("Hello"));
    }

    #[test]
    fn next_bytes_returns_error_if_not_simple_or_bulk_frame() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Null])).unwrap();
        assert!(parser.next_bytes().is_err());
    }

    #[test]
    fn call_finish_on_an_empty_frame_array_returns_ok() {
        let mut parser = Parser::new(Frame::Array(vec![])).unwrap();
        assert!(parser.finish().is_ok());
    }

    #[test]
    fn call_finish_on_a_nonempty_frame_array_returns_err() {
        let mut parser = Parser::new(Frame::Array(vec![Frame::Null])).unwrap();
        assert!(parser.finish().is_err());
    }
}
