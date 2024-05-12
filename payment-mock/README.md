# Simple Payment Api

This API was created to mock the payment api in another project

## Endpoints

### /payment/

Request payload:

```json
{
	"webhook_url": "https://yourwebhookurl.com",
	"value": 100.00
}
```

Response payload - 200:

```json
{
	"message": "success",
	"payment_code": "889d3c35-c4b9-4adb-97a1-fd8e5d89cf58"
}
```