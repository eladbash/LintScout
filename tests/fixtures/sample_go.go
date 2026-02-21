package main

func main() {
    x := 1 //nolint:errcheck
    _ = x  //#nosec G101
    //lint:ignore SA1000 reason
}
