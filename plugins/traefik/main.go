package traefik

import (
	"bytes"
	"context"
	"fmt"
	"net/http"
	"text/template"
    "github.com/traefik/traefik/v3/pkg/middlewares"
)

// ============================== CONFIG ==============================

type Config struct {
	WolonreqUrl      string `yaml:"wolonreqUrl"`
}

func CreateConfig() *Config {
	return &Config{
        WolonreqUrl: "http://localhost:8080", // TODO: set decent default url
	}
}

func (c *Config) Validate() error {
    if len(c.WolonreqUrl) == 0 {
        return fmt.Errorf("wolonreqUrl cannot be empty")
    }
    return nil
}

// ============================ MIDDLEWARE ============================

type WolonreqMiddleware struct {
	next     http.Handler
	name     string
}

func New(ctx context.Context, next http.Handler, config *Config, name string) (http.Handler, error) {
	middlewares.GetLogger(ctx, name, name).Debug().Msg("Creating middleware")

    if err := config.Validate(); err != nil {
        return nil, err
    }

	return &WolonreqMiddleware{
		next:     next,
		name:     name,
	}, nil
}

func (a *WolonreqMiddleware) ServeHTTP(rw http.ResponseWriter, req *http.Request) {
	middlewares.GetLogger(ctx, a.name, a.name).Debug().Msg("Received request")

	a.next.ServeHTTP(rw, req)
}