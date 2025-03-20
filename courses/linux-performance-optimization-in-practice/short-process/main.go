//go:build linux

package main

import (
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
)

func main() {
	_ = os.Mkdir("/tmp/cannot-write", 0555)
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		cmd := exec.Command("stress", "-t", "1", "-d", "1")
		cmd.Dir = "/tmp/cannot-write"
		if output, err := cmd.CombinedOutput(); err != nil {
			log.Printf("stress failed: %v, output: %s", err, output)
		}
		io.WriteString(w, "It works!\n")
	})

	if err := http.ListenAndServe(":8080", nil); err != nil {
		panic(err)
	}
}
