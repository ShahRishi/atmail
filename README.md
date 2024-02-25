# atmail

_Simple, fast filesharing_

## Get started

### Download the latest release (Mac)
Download the latest release for your system under the releases

### Unzip the release
`tar -xvf ~/_downloaded_file.tar.gz`

_Note: if you have issues here, delete and redownload the zip_

### Move file to executable path
`sudo mv ~/Downloads/dust /usr/local/bin/`


## Usage
First set up your gmail app key, as seen [here](https://support.google.com/accounts/answer/185833?hl=en)

Once you have your app password, configure your email login as shown (you only need to do this once!):

`atmail config <email> <token>`

Try sending an email with a file to an email of your choice by:

`atmail send <email> <path>`
