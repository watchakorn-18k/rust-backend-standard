use fldp_rust_backend_template::utils::pagination::PaginationResult;

#[test]
fn test_pagination_result_calculation() {
    let data = vec![1, 2, 3];
    let result = PaginationResult::new(data, 1, 10, 25);
    
    assert_eq!(result.total_pages, 3);
    assert_eq!(result.page, 1);
    assert_eq!(result.limit, 10);
}
