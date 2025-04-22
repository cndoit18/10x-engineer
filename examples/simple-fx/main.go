package main

import (
	"context"
	"fmt"
	"io"
	"log/slog"
	"net"
	"net/http"
	"os"

	"go.uber.org/fx"
	"go.uber.org/fx/fxevent"
)

func main() {
	fx.New(
		fx.Provide(
			NewHTTPServer,
			func() slog.Handler {
				return slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{})
			},
			slog.New,
			fx.Annotate(
				NewServeMux,
				fx.ParamTags(`group:"routes"`),
			),

			fx.Annotate(
				Route("/hello", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					body, err := io.ReadAll(r.Body)
					if err != nil {
						http.Error(w, "Internal server error", http.StatusInternalServerError)
						return
					}

					if _, err := fmt.Fprintf(w, "Hello, %s\n", body); err != nil {
						http.Error(w, "Internal server error", http.StatusInternalServerError)
						return
					}
				})),
				fx.ResultTags(`group:"routes"`),
			),

			fx.Annotate(
				Route("/echo", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					if _, err := io.Copy(w, r.Body); err != nil {
						fmt.Fprintln(os.Stderr, "Failed to handle request:", err)
					}
				})),
				fx.ResultTags(`group:"routes"`),
			),
		),

		fx.WithLogger(func(log *slog.Logger) fxevent.Logger {
			return &fxevent.SlogLogger{Logger: log}
		}),

		fx.Invoke(func(*http.Server) {}),
	).Run()
}

func NewServeMux(routes ...route) *http.ServeMux {
	mux := http.NewServeMux()
	for _, r := range routes {
		r(mux)
	}
	return mux
}

type route func(mux *http.ServeMux)

func Route(pattern string, handler http.Handler) func() route {
	return func() route {
		return func(mux *http.ServeMux) {
			mux.Handle(pattern, handler)
		}
	}
}

func NewHTTPServer(lc fx.Lifecycle, log *slog.Logger, mux *http.ServeMux) *http.Server {
	srv := &http.Server{
		Handler: mux,
		Addr:    ":8080",
	}
	lc.Append(fx.StartStopHook(
		func(ctx context.Context) error {
			ln, err := (&net.ListenConfig{}).Listen(ctx, "tcp", srv.Addr)
			if err != nil {
				return fmt.Errorf("failed to listen on %s: %w", srv.Addr, err)
			}
			go srv.Serve(ln)
			return nil
		}, func(ctx context.Context) error {
			return srv.Shutdown(ctx)
		},
	))
	return srv
}
