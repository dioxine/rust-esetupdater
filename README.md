# rust-esetupdater
This small app can parse updates for ESET AV protection products, download modules selectively and build own brand new 
structured update.ver file you can use on your own mirror. 

Currently works only with custom mirrors with public access. BasicAuth will be added after i finish pretty looking progress bar of downloading process. Also in close future i will add filtering for desired or chosen architectures, but code is very simple and well commented, you can add your own logic by yourself, welcome.

Rename config.toml.example to config.toml, read comments in it, edit, change ```host``` to the actual server you have access and you are ready to go! Also supports command line directives so you can use programm without config file. Only ```host``` and ```root_dir``` parameters are accessible via command line. ```user-agent``` in that case will be used built-it.

```-h``` will show you all variants of arguments. 

Feel free to test it. Will be very glad for reviews!
