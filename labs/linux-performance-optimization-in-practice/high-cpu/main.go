package main

import (
	"io"
	"math"
	"net/http"
	_ "net/http/pprof"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		x := 0.0
		for i := 0; i < 1000000000; i++ {
			x += math.Sqrt(x)
		}

		io.WriteString(w, "It works!\n")
	})

	if err := http.ListenAndServe(":8080", nil); err != nil {
		panic(err)
	}
}
