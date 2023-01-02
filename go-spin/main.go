package main

import (
	"fmt"
	"net/http"
	"net/http/httputil"
	"os"
	"runtime"

	spinhttp "github.com/fermyon/spin/sdk/go/http"
)

func init() {
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		reqDump, err := httputil.DumpRequest(r, true)
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			fmt.Fprintln(w, err.Error())
			return
		}
		w.Header().Set("Content-Type", "text/plain")
		fmt.Fprintln(w, string(reqDump))
		fmt.Fprintln(w, fmt.Sprintf("arch: %s", runtime.GOARCH))
		fmt.Fprintln(w, fmt.Sprintf("os: %s", runtime.GOOS))
		fmt.Fprintln(w, fmt.Sprintf("version: %s", os.Getenv("VERSION")))
	})
}

func main() {}
