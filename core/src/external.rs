use primitive_types::H160;

/// Operations for recording external costs
pub enum ExternalOperation {
	/// Reading basic account from storage. Fixed size.
	AccountBasicRead,
	/// Reading address code from storage. Dynamic size.
	AddressCodeRead(H160),
	/// Basic check for account emptiness. Fixed size.
	IsEmpty,
	/// Writing to storage. Fixed size.
	Write,
}
