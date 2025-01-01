package main

import (
	"embed"
	"errors"
	"fmt"
	"log"
	"net"
	"net/http"
	"time"

	"github.com/webview/webview_go"
)

//go:embed static/*
var embeddedFiles embed.FS

func main() {
	fileServer := http.FileServer(http.FS(embeddedFiles))
	mux := http.NewServeMux()
	mux.Handle("/static/", fileServer)
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/" {
			data, err := embeddedFiles.ReadFile("static/index.html")
			if err != nil {
				http.Error(w, "index.html not found", http.StatusInternalServerError)
				return
			}
			w.Header().Set("Content-Type", "text/html")
			w.Write(data)
			return
		}
		fileServer.ServeHTTP(w, r)
	})

	listener, err := net.Listen("tcp", "127.0.0.1:7000")
	if err != nil {
		log.Fatalf("Port listening failed: %v", err)
	}
	defer listener.Close()

	server := &http.Server{
		Handler: mux,
	}
	go func() {
		if err := server.Serve(listener); err != nil && !errors.Is(err, http.ErrServerClosed) {
			log.Fatalf("Failed to start HTTP server: %v", err)
		}
	}()
	defer server.Close()

	addr := listener.Addr().String()
	url := fmt.Sprintf("http://%s/", addr)

	timeout := time.After(2 * time.Second)
	tick := time.Tick(10 * time.Millisecond)
	for {
		select {
		case <-timeout:
			log.Fatalf("Server timed out.")
		case <-tick:
			conn, err := net.Dial("tcp", addr)
			if err != nil {
				continue
			}
			conn.Close()
			goto ServerUp
		}
	}
ServerUp:

	w := webview.New(true)
	defer w.Destroy()
	w.SetTitle("happy new year!")
	w.SetSize(800, 600, webview.HintNone)
	w.Navigate(url)
	w.Run()
}
