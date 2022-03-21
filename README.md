# crypto_signal_tracker
A simple utility for tracking prices of certain cryptocurrencies

The project needs more error-handling with what it already has, but more features would serve it well:  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; an easy way to add new currencies without editing code (e.g., from stdin)  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; a database to store time-series data  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; more advanced signals  

# Usage
As is in the master branch, clone with the following command:  

```git clone https://github.com/0xgb1/crypto_signal_tracker```  

change directories to the package:  

```cd crypto_signal_tracker/```  

with cargo installed build the package in the new directory:  

``` cargo build```  

and finally run:  

```cargo run```  

The program has a predefined set of currencies of interest. To add currencies, change the available maps in ```src/values.rs```.
