package signature

const replDeployPublicKey = `-----BEGIN RSA PUBLIC KEY-----
MIIECgKCBAEAswkDXZlAqW1UpGiLFBW1ohSvUIqqcwrOt1ubbWrltrYT+3SQV24C
Su9j93+DX9tsFBuVDE3DSutddBmdWh0zFxDdSO+uA8JBJki9GfHNoynFcPLl3AxA
4iUh6nD6uSdXIGkJaJ+U8/Jix2AXS7Qk5Jfoktx88GtKoHAwznmfxdJwrFeiX8D8
Lqh34enh7pnntMp0vrpiTHu37H/VPGEAWkFoHuQMLoaHPgzF/Nk8NsjL2Uzvp8+Z
Vda8cXk2DeEm0x4q6kCWwchEcZF2jHcARjQ7ov7Vh5qZzlXcODt6i7NWUFX5h6g4
IodZXteh9apPaWSwXuMO+vCM3peYYfpFgVf/u2rh+wH6PjDiZE+keoA2PkPfvxVg
BUL54z6EYMR5pItN5MIqFigqBqUcrmoQhtwMZyU/bAVjqTjXa1pyE1wn18h1ufFf
6WXY/poVnmru+iA6IYG/D5YAolombTfA9U74qF1LWCIkahoNKjtX7cHRFDRT9OCo
inCiWiVG9WAbxMDU08j1CEut/yXhpSx8J4p878+LMapFChs7yIYV6TDS5UELKtBz
Ij6XWQKzT/PtwCYTxlZ+PlgMQw5ybG2imFzFy7JJpADkgWHGIn2j7Gzqo+DxcVC4
lotNBlZQTy5SVq+x6KwdJPG9+a6ECSiv7W+yyBh8QBPcC7oJAFdngSuvaE12TZvO
myRA05TX/Ron4/s0FbMrrP2K4oSuaCX6WlGcHcLNXz8OX0Egzyg3KKh5umzH8Ce9
ORoPwubbzXfZpbUGQb+iF8GPEp14z7VsDivjvzB/gaDqZ6+wSnPR6U+dk4SmP+Uk
/4Dc6ICxqct/BJOTMm9Fagp5mRcjXrTJ2TM+1ZKd/8lwL+gdcEYiNbb65d0ESN/1
qFWcjdihPqKjmn/5+PUdSl+wYfdbfnaT6fL01cOm/3xRS3l2A+9G5Bfh0PCdrg+A
+qKkGUp9cRD1w53ZS3zv/AmhY5e1VPc3mggpGn3uSseAc1NY5facH8ziiNfXLhQp
mjnOO5EsSjiXBXJ4uBisAbtiAaYELXYHOR1qf8catdI7jyUplCMpmqKT5ebUuhyh
6IP54Zx0YPznqwJSKJrPDoIxiD7iePQq0tOhxnMfGT8xeDZkTZ9sdgzbyqOnthX3
PUN9Kexr5nSWWfb0AJRTaZBxiXx4SKdo2yw6aaoIAOo6SyJLm0u0Qwa5Xm7GG0NS
0LsYDDPt/NNu+0tztpJM5DU6eRKePj9Lx8Xn8Hku3HqVR2LleSIyk7Z0G5yTZwdM
+9P0tsivT3+qKNy4BGin8mSBOCixhrL2YnNK5pOHrCXot562HTFKgvYz35u6sS6L
yggLIsW8CUnOIhj0AKovh9OvyC//N/GRLQIDAQAB
-----END RSA PUBLIC KEY-----`

const (
	// Strings
	sReplitDeployJsonPath = "./replit-deploy.json"

	// Errors
	sInvalidSignatureError  = "Invalid Signature"
	sPrivateKeyParseError   = "Couldn't parse public key, open a new issue"
	sBadPayloadError        = "Bad payload"
	sSignatureTooOldError   = "Signature too old"
	sBadEndpointError       = "Signed request not intended for current endpoint"
	sMissingConfigFileError = "Config file doesn't exist"
	sInvalidJSONError       = "Invalid config JSON"
)
