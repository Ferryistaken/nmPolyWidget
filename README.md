# nmPolyWidget
A rust program which prints information about a specified interface using Network Manager. Mainly used as a NetworkManager Polybar widget


## Installation
Make sure that ```cargo``` is intalled.

```git clone https://github.com/Ferryistaken/nmPolyWidget.git```
```cd nmPolyWidget/```
```cargo build --release```
```sudo mv target/release/nmPolyWidget /usr/bin/```
Then just run ```nmPolyWidget <interface>``` to get data about a specified interface

