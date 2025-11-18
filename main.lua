local MODULE_PATH = [[;.\src\?.lua]]

if not string.endsWith(package.path, MODULE_PATH) then
    package.path = package.path .. MODULE_PATH
end

local app = require('app')

app.create()
