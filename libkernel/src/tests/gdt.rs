#[test_case]
#[allow(non_snake_case)]
pub fn set_base__32bit_address__written_correctly() {
    use crate::gdt::SegmentDescriptor;

    // Arrange
    let mut segment: SegmentDescriptor = SegmentDescriptor::new();
    let offset: u32 = 0xAABBCCCC;
    
    // Act
    let new_segment = segment.set_base(offset);

    // Assert
    assert_eq!(new_segment.pointer_high, 0xAA);
    assert_eq!(new_segment.pointer_middle, 0xBB);
    assert_eq!(new_segment.pointer_low, 0xCCCC);
}

#[test_case]
#[allow(non_snake_case)]
pub fn set_limit__20bit_number__written_correctly() {
    use crate::gdt::SegmentDescriptor;

    // Arrange
    let mut segment: SegmentDescriptor = SegmentDescriptor::new();
    let limit: u32 = 0xABBBB;
    
    // Act
    let new_segment = segment.set_limit(limit);

    // Asserts
    assert_eq!(new_segment.limit, 0xBBBB);
    assert_eq!(new_segment.options, 0xA);
}