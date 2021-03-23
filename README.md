# sfi-web

A web-based frontend for sfi (short for Shared Food Inventory), which manages shared resources with associated expiry dates, optimized for occasionally connected computing scenarios.

(todo)

## Development

Run `trunk serve -d public` to simply serve the static frontend files.

If you're developing sfi you'll probably want to use the sfi-workspace (found in the sfi-core repository) and to run `trunk watch -d public` (instead of serve) together with `cargo watch -x run` in sfi-server.

As of right now, `trunk` emits colorless output. If this bothers you, consider running `cargo watch` (which is equivalent to `cargo watch -x check`) in this repository as well to get colored output in addition to automatic re-compilation.

## Licence & Copyright

Copyright (c) 2021 Bernd-L. All rights reserved.

![AGPL v3: Free as in Freedom](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)

sfi-web is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

sfi-web is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with sfi-web. If not, see <https://www.gnu.org/licenses/>.

This project (including its source code and its documentation) is released under the terms of the [GNU Affero General Public License](/LICENSE.md).
