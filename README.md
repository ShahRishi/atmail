# atmail

_Simple, fast filesharing with Gmail_

## Installation

### Mac
**_Download the latest release_**

Download the latest release for your system under the releases

**_Unzip the release_**

`unzip -r ~/Downloads/atmail-[arm/intel].zip`

_Note: if you have issues here, delete and redownload the zip_

**_Move file to executable path_**
`sudo mv ~/Downloads/atmail-[arm/intel]/atmail /usr/local/bin/`


### Windows 
docs updating soon


## Usage
_**Set up app password**_
First set up your gmail app app password, as seen [here](https://support.google.com/accounts/answer/185833?hl=en)

_**Configure email login (done only for first use!)**_

Once you have your app password, configure your email login as shown:

`atmail config <email> <token>`

Ex: `atmail config me@gmail.com xxs\ dgsds\ sdfsdf` 

_**Send an attachment**_

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

