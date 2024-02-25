# atmail

_Simple, fast filesharing with Gmail_

## Installation

### Unix
** Download the latest release**
Download the latest release for your system under the releases

Unzip the release
`tar -xvf ~/_downloaded_file.tar.gz`

_Note: if you have issues here, delete and redownload the zip_

**Move file to executable path**
`sudo mv ~/Downloads/dust /usr/local/bin/`


### Windows 
docs updating soon


## Usage
**Set up app password**
First set up your gmail app app password, as seen [here](https://support.google.com/accounts/answer/185833?hl=en)

**Configure email login (done only for first use!)**
Once you have your app password, configure your email login as shown:

`atmail config <email> <token>`

Ex: `atmail config me@gmail.com xxs\ dgsds\ sdfsdf` 

**Send an attachment**
Try sending an email with a file to an email of your choice by:

`atmail send <email> <path>`

Ex: `atmail send me@gamil.com ./images/image.png`

## Uninstall 
To uninstall, simply remove the executable and config directory 

To remove executable:
`sudo rm /usr/local/bin/atmail`

To remove the config directory (OSX):
`sudo rm -rf ~/Library/Application\ Support/atmail`

To remove the config directory (Ubuntu):
`sudo rm -rf ~/.config/atmail`

