# golibra

```
# install dependencies
$ go get -u google.golang.org/grpc
$ go get -u github.com/golang/protobuf/protoc-gen-go

# extract the .proto's from libra and compile to go
$ ./gen_protos.sh

# make a basic request to a testnet node
$ go run libra_client/main.go
```

## Subtree

```
git subtree add --prefix=sub/libra libra master --squash
git subtree pull --prefix=sub/libra libra master --squash
git subtree push --prefix=sub/libra libra master
```
