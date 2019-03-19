# playbar / password

The `password` crate shall be the only crate which actually sees your Google password or tokens.

This crate asks for your email and password and returns back a `SJToken` which is an Access Token tied to your account that can only be used to access Google SkyJam services (Google Play Music).
