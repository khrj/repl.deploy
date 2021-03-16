package signature

type Payload struct {
	Timestamp int    `json:"timestamp" validate:"required,numeric"`
	Endpoint  string `json:"endpoint" validate:"required,url"`
}

type Config struct {
	Endpoint string `json:"endpoint" validate:"required,url"`
}

type ValidationResult struct {
	Body    string `json:"body"`
	Status int    `json:"status"`
}
