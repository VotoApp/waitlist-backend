# Voto Waitlist Backend

This is a pure rust package that utilizes the experimental rust lambda runtime
to mediate between AWS and application code. In this case, the application
receives an email as a string in a HTTP POST request, validates the email,
confirms that the email doesn't already exist in the dynamodb table, and then
commits it to the table. This provides a very raw "email" wailist that lends
itself to ease of use in the future. The email record is accompanied by a
"DateReceived" number in Epoch seconds that captures when the service received
this email.  This can be useful for any DSAR/IDD as emails are generally
considered "Personally Identifiable Information" (PII) and may need be returned
or deleted at a later date.

## Empirical Measurements

The cold start time for this lambda is ~250ms with a 'primed' time of ~50ms