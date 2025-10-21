# rust-esetupdater
This small app can parse updates for ESET AV protection products, download modules selectively and build own brand new 
structured update.ver file you can use on your own mirror. 

BasicAuth is working now.

Rename config.toml.example to config.toml, read comments in it, edit, change ```host``` to the actual server,
then in advance, you might be know ```username``` and ```password```  of that resource.
If endpoint does not even has BasicAuth, and you entered them in config.toml, no problems, they will be ignored.

Redirections at this point are not working! If your server is accessible only with https, you SHOULD set https-address
in config.toml 

Functionality of non-standard ports, like https://example.com:12345 i have not tested, but should work. TELL ME!

So, if you have got access to server - you are ready to go! 

Also supports command line directives so you can use programm without config file. Only ```host``` and ```root_dir``` parameters are accessible via command line. ```user-agent``` in that case will be used built-it.

```-h``` will show you all variants of arguments. 


PLEASE!!! Be carefull in config.toml
i mean NO overslashes! No beforingslashes, just dolike a wrote in examples, please

Feel free to test it. Will be very glad for reviews!

TODO: Sanitaze that slash shit, fixing it to URI scruct from the beneath
