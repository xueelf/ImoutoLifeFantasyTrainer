local function endsWith(text, suffix)
    return text:find(suffix, nil, true) ~= nil
end

local module_path = [[;.\src\?.lua]]

if not endsWith(package.path, module_path) then
    package.path = package.path .. module_path
end

local app = require('app')

app.create()
