local app = require('config.app')

local ROOT = 'assets/'

local function loadFile(filename)
    local stream = createMemoryStream()

    stream.loadFromFile(ROOT .. filename)
    return stream
end

local function loadFont(filename)
    local base = 'fonts/'
    local extension = '.ttf'
    local path = base .. filename .. extension
    local stream = loadFile(path)

    loadFontFromStream(stream)
end

local function playVoice(filename)
    local base = 'sounds/'
    local extension = '.wav'
    local path = base .. filename .. extension
    local stream = loadFile(path)

    playSound(stream)
end

local function getRandomFileByFolder(path, exclude)
    local files = getFileList(path)

    if not exclude then
        return files[math.random(1, #files)]
    end
    local candidate = {}

    for _, file in ipairs(files) do
        if not string.endsWith(file, exclude) then
            table.insert(candidate, file)
        end
    end
    return candidate[math.random(1, #candidate)]
end

local function resolvePointer(offsets)
    local name = app.client[tutorial.platform].name
    local offset = app.client[tutorial.platform].offset
    local address = getAddress(name .. '+' .. string.format('%X', offset))
    local pointer = readPointer(address)

    for i = 1, #offsets - 1 do
        pointer = readPointer(pointer + offsets[i])
    end
    return pointer + offsets[#offsets]
end

local function getPointerValue(offsets)
    local address = resolvePointer(offsets)
    local value = readInteger(address)

    return value
end

local function setPointerValue(offsets, value)
    local address = resolvePointer(offsets)
    writeInteger(address, value)
end

local function prompt(title, offsets)
    local address = resolvePointer(offsets)
    local value = readInteger(address)
    local input = inputQuery(title, '请输入新的数值：', value)

    if input ~= nil then
        writeInteger(address, input)
    end
    return input
end

return {
    loadFont = loadFont,
    playVoice = playVoice,
    getRandomFileByFolder = getRandomFileByFolder,
    getPointerValue = getPointerValue,
    setPointerValue = setPointerValue,
    prompt = prompt,
}
