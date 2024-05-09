# TEST SMTP SERVER for EMAIL SENDING

A simple utility to test the SMTP server for sending emails.

### Help
```
$ ./sendmail-test --help
Send test Email!

Usage: sendmail-test [OPTIONS] --smtp-host <SMTP-HOST> --smtp-user <SMTP-USER> --smtp-password <SMTP-PASSWORD> --from-email <FROM-EMAIL> --to-email <TO-EMAIL>

Options:
  -h, --smtp-host <SMTP-HOST>
          SMTP Host

      --port <SMTP-PORT>
          SMTP Port

      --hello_name <SMTP-HELLO-NAME>
          SMTP Hello name

  -u, --smtp-user <SMTP-USER>
          SMTP User

  -p, --smtp-password <SMTP-PASSWORD>
          SMTP Password

  -f, --from-email <FROM-EMAIL>
          From Email

  -t, --to-email <TO-EMAIL>
          To Email

  -s, --subject <SUBJECT>
          Default is None, if none is provided, it will use the default email subject

  -b, --body <BODY>
          Default is None, if none is provided, it will use the default email body

      --use-ses
          USE Amazon SES Service, if yes!

      --help


  -V, --version
          Print version
```

### Example
```
./sendmail -h <SMTP-SERVER-HOST> -u <SMTP-SERVER-USER> -p <SMTP-SERVER-PASSWORD> -f <FROM-EMAIL> -t <TO-EMAIL>
```

---
License: MIT
