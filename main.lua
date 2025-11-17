local function endsWith(text, suffix)
    return text:find(suffix, nil, true) ~= nil
end

local MODULE_PATH = [[;.\src\?.lua]]

if not endsWith(package.path, MODULE_PATH) then
    package.path = package.path .. MODULE_PATH
end

local app = require('app')

app.create()
