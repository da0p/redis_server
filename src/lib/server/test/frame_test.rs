#[cfg(test)]
mod frame_test {
    use super::super::*;

    #[test]
    fn call_new_create_an_empty_frame() {
        let frame = Frame::new();
        assert_eq!(frame, Frame::Array(vec![]));
    }

    #[test]
    fn push_integer_into_frame() {
        let mut frame = Frame::new();
        frame.push_int(42);
        assert_eq!(frame, Frame::Array(vec![Frame::Integer(42)]));
    }

    #[test]
    fn push_bytes_into_frame() {
        let mut frame = Frame::new();
        frame.push_bulk(Bytes::from("HelloWorld!"));
        assert_eq!(
            frame,
            Frame::Array(vec![Frame::Bulk(Bytes::from("HelloWorld!"))])
        );
    }

    #[test]
    fn check_null_frame() {
        let bytes = String::from("$-1\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn parse_null_frame() {
        let bytes = String::from("$-1\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(Frame::parse(&mut frame).unwrap(), Frame::Null);
    }

    #[test]
    fn check_simple_array_bulk_frame() {
        let bytes = String::from("*1\r\n$4\r\nping\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok())
    }

    #[test]
    fn parse_simple_array_bulk_frame() {
        let bytes = String::from("*1\r\n$4\r\nping\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Array(vec![Frame::Bulk(Bytes::from("ping"))])
        );
    }

    #[test]
    fn parse_simple_bulk_frame() {
        let bytes = String::from("$11\r\nhello world\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Bulk(Bytes::from("hello world"))
        );
    }

    #[test]
    fn check_echo_string_frame() {
        let bytes = String::from("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn parse_echo_string_frame() {
        let bytes = String::from("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Array(vec![
                Frame::Bulk(Bytes::from("echo")),
                Frame::Bulk(Bytes::from("hello world"))
            ])
        );
    }

    #[test]
    fn check_simple_frame_with_space() {
        let bytes = String::from("+hello world\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn parse_simple_frame_with_space() {
        let bytes = String::from("+hello world\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Simple(String::from("hello world"))
        );
    }

    #[test]
    fn check_simple_frame_no_space() {
        let bytes = String::from("+OK\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn parse_simple_frame_no_space() {
        let bytes = String::from("+OK\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Simple(String::from("OK"))
        );
    }

    #[test]
    fn check_error_frame() {
        let bytes = String::from("-Error message\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn parse_error_frame() {
        let bytes = String::from("-Error message\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Error(String::from("Error message"))
        );
    }

    #[test]
    fn check_empty_bulk_string() {
        let bytes = String::from("$0\r\n\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn parse_empty_bulk_string() {
        let bytes = String::from("$0\r\n\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert_eq!(
            Frame::parse(&mut frame).unwrap(),
            Frame::Bulk(Bytes::from(""))
        );
    }

    #[test]
    fn check_get_command() {
        let bytes = String::from("*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_ok());
    }

    #[test]
    fn check_missing_command() {
        let bytes = String::from("*2\r\n$3\r\nget\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_err());
    }

    #[test]
    fn parse_missing_command() {
        let bytes = String::from("*2\r\n$3\r\nget\r\n");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::parse(&mut frame).is_err());
    }

    #[test]
    fn check_missing_new_line() {
        let bytes = String::from("*1\r\n$3\r\nget");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::check(&mut frame).is_err());
    }

    #[test]
    fn parse_missing_new_line() {
        let bytes = String::from("*1\r\n$3\r\nget");
        let mut frame = Cursor::new(bytes.as_bytes());
        assert!(Frame::parse(&mut frame).is_err());
    }
}
