// Code generated by protoc-gen-go. DO NOT EDIT.
// source: mempool_status.proto

package mempool

import (
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type MempoolAddTransactionStatusCode int32

const (
	// Transaction was sent to Mempool
	MempoolAddTransactionStatusCode_Valid MempoolAddTransactionStatusCode = 0
	// The sender does not have enough balance for the transaction.
	MempoolAddTransactionStatusCode_InsufficientBalance MempoolAddTransactionStatusCode = 1
	// Sequence number is old, etc.
	MempoolAddTransactionStatusCode_InvalidSeqNumber MempoolAddTransactionStatusCode = 2
	// Mempool is full (reached max global capacity)
	MempoolAddTransactionStatusCode_MempoolIsFull MempoolAddTransactionStatusCode = 3
	// Account reached max capacity per account
	MempoolAddTransactionStatusCode_TooManyTransactions MempoolAddTransactionStatusCode = 4
	// Invalid update. Only gas price increase is allowed
	MempoolAddTransactionStatusCode_InvalidUpdate MempoolAddTransactionStatusCode = 5
)

var MempoolAddTransactionStatusCode_name = map[int32]string{
	0: "Valid",
	1: "InsufficientBalance",
	2: "InvalidSeqNumber",
	3: "MempoolIsFull",
	4: "TooManyTransactions",
	5: "InvalidUpdate",
}

var MempoolAddTransactionStatusCode_value = map[string]int32{
	"Valid":               0,
	"InsufficientBalance": 1,
	"InvalidSeqNumber":    2,
	"MempoolIsFull":       3,
	"TooManyTransactions": 4,
	"InvalidUpdate":       5,
}

func (x MempoolAddTransactionStatusCode) String() string {
	return proto.EnumName(MempoolAddTransactionStatusCode_name, int32(x))
}

func (MempoolAddTransactionStatusCode) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_cad4a86f8a5465be, []int{0}
}

type MempoolAddTransactionStatus struct {
	Code                 MempoolAddTransactionStatusCode `protobuf:"varint,1,opt,name=code,proto3,enum=mempool.MempoolAddTransactionStatusCode" json:"code,omitempty"`
	Message              string                          `protobuf:"bytes,2,opt,name=message,proto3" json:"message,omitempty"`
	XXX_NoUnkeyedLiteral struct{}                        `json:"-"`
	XXX_unrecognized     []byte                          `json:"-"`
	XXX_sizecache        int32                           `json:"-"`
}

func (m *MempoolAddTransactionStatus) Reset()         { *m = MempoolAddTransactionStatus{} }
func (m *MempoolAddTransactionStatus) String() string { return proto.CompactTextString(m) }
func (*MempoolAddTransactionStatus) ProtoMessage()    {}
func (*MempoolAddTransactionStatus) Descriptor() ([]byte, []int) {
	return fileDescriptor_cad4a86f8a5465be, []int{0}
}

func (m *MempoolAddTransactionStatus) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_MempoolAddTransactionStatus.Unmarshal(m, b)
}
func (m *MempoolAddTransactionStatus) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_MempoolAddTransactionStatus.Marshal(b, m, deterministic)
}
func (m *MempoolAddTransactionStatus) XXX_Merge(src proto.Message) {
	xxx_messageInfo_MempoolAddTransactionStatus.Merge(m, src)
}
func (m *MempoolAddTransactionStatus) XXX_Size() int {
	return xxx_messageInfo_MempoolAddTransactionStatus.Size(m)
}
func (m *MempoolAddTransactionStatus) XXX_DiscardUnknown() {
	xxx_messageInfo_MempoolAddTransactionStatus.DiscardUnknown(m)
}

var xxx_messageInfo_MempoolAddTransactionStatus proto.InternalMessageInfo

func (m *MempoolAddTransactionStatus) GetCode() MempoolAddTransactionStatusCode {
	if m != nil {
		return m.Code
	}
	return MempoolAddTransactionStatusCode_Valid
}

func (m *MempoolAddTransactionStatus) GetMessage() string {
	if m != nil {
		return m.Message
	}
	return ""
}

func init() {
	proto.RegisterEnum("mempool.MempoolAddTransactionStatusCode", MempoolAddTransactionStatusCode_name, MempoolAddTransactionStatusCode_value)
	proto.RegisterType((*MempoolAddTransactionStatus)(nil), "mempool.MempoolAddTransactionStatus")
}

func init() { proto.RegisterFile("mempool_status.proto", fileDescriptor_cad4a86f8a5465be) }

var fileDescriptor_cad4a86f8a5465be = []byte{
	// 264 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x84, 0x90, 0x3d, 0x4b, 0x03, 0x41,
	0x10, 0x86, 0xbd, 0x98, 0x18, 0xb2, 0xa0, 0xac, 0x6b, 0xc0, 0x03, 0x0b, 0x83, 0x8d, 0xa7, 0xc5,
	0x1d, 0x68, 0x25, 0xd8, 0x18, 0x41, 0xb8, 0x22, 0x16, 0x49, 0xb4, 0xb0, 0x09, 0x73, 0xb7, 0x93,
	0x64, 0x61, 0xbf, 0xbc, 0xdd, 0x15, 0xfd, 0x2b, 0xfe, 0x5a, 0xb9, 0x73, 0x05, 0x2b, 0xed, 0x66,
	0x86, 0xf7, 0x79, 0x06, 0x5e, 0x32, 0x56, 0xa8, 0xac, 0x31, 0x72, 0xe5, 0x3c, 0xf8, 0xe0, 0x72,
	0xdb, 0x18, 0x6f, 0xd8, 0x30, 0x5e, 0xcf, 0x02, 0x39, 0x99, 0x7d, 0x8f, 0x77, 0x9c, 0x2f, 0x1b,
	0xd0, 0x0e, 0x6a, 0x2f, 0x8c, 0x5e, 0x74, 0x69, 0x76, 0x4b, 0xfa, 0xb5, 0xe1, 0x98, 0x26, 0x93,
	0x24, 0x3b, 0xb8, 0xca, 0xf2, 0x88, 0xe5, 0x7f, 0x30, 0xf7, 0x86, 0xe3, 0xbc, 0xa3, 0x58, 0x4a,
	0x86, 0x0a, 0x9d, 0x83, 0x0d, 0xa6, 0xbd, 0x49, 0x92, 0x8d, 0xe6, 0x3f, 0xeb, 0xe5, 0x67, 0x42,
	0x4e, 0xff, 0x71, 0xb0, 0x11, 0x19, 0x3c, 0x83, 0x14, 0x9c, 0xee, 0xb0, 0x63, 0x72, 0x54, 0x6a,
	0x17, 0xd6, 0x6b, 0x51, 0x0b, 0xd4, 0x7e, 0x0a, 0x12, 0x74, 0x8d, 0x34, 0x61, 0x63, 0x42, 0x4b,
	0xfd, 0xd6, 0xa6, 0x16, 0xf8, 0xfa, 0x18, 0x54, 0x85, 0x0d, 0xed, 0xb1, 0x43, 0xb2, 0x1f, 0xe5,
	0xa5, 0x7b, 0x08, 0x52, 0xd2, 0xdd, 0xd6, 0xb0, 0x34, 0x66, 0x06, 0xfa, 0xe3, 0xd7, 0x33, 0x47,
	0xfb, 0x6d, 0x36, 0x1a, 0x9e, 0x2c, 0x07, 0x8f, 0x74, 0x30, 0xbd, 0x78, 0x39, 0xdf, 0x08, 0xbf,
	0x0d, 0x55, 0x5e, 0x1b, 0x55, 0xd8, 0xad, 0x14, 0xf6, 0xa6, 0x90, 0xa2, 0x6a, 0x60, 0x85, 0xef,
	0xa0, 0xac, 0xc4, 0x22, 0xf6, 0x50, 0xed, 0x75, 0x75, 0x5e, 0x7f, 0x05, 0x00, 0x00, 0xff, 0xff,
	0x30, 0x62, 0xb6, 0x36, 0x66, 0x01, 0x00, 0x00,
}
