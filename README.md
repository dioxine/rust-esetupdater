# rust-esetupdater
This small app can parse updates for ESET AV protection products, download modules selectively and build own brand new 
structured update.ver file you can use on your own mirror. 

BasicAuth is working now.

Rename config.toml.example to config.toml, read comments in it, edit, change ```host``` to the actual server,
then in advance, you might be know ```username``` and ```password```  of that resource.
If endpoint does not even has BasicAuth, and you entered them in config.toml, no problems, they will be ignored.

Redirections at this point are not working! If your server is accessible only with https, you SHOULD set https-address
in config.toml 

Functionality of non-standard ports, like https://example.com:12345 i have not tested, but should work.

TODO: Sanitaze paths.
