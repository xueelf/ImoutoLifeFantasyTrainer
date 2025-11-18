local cheat = require('cheat')
local config = require('config')
local keymap = require('keymap')

local Feature = createListView(Window)
local Prompt = createPanel(Window)

local FEATURE_WIDTH = 400
local header = {
    { caption = '状态', width = 45 },
    { caption = '热键', width = 45 },
    { caption = '功能', width = 140 },
    { caption = '描述', width = 170 },
}
local status = {
    enable = '●',
    disable = '○',
}

local pool = {}

local function addShortcut(owner, code, action)
    if not owner then
        return
    end
    local callback = function()
        local method = cheat[action]

        if not method then
            showMessage(string.format('Action "%s" is not defined', action))
        else
            local enabled = owner.caption == status.disable
            local success, result = pcall(method, enabled)

            if success then
                owner.caption = enabled and status.enable or status.disable
            else
                showMessage(result)
            end
        end
    end
    createHotkey(callback, code)
    table.insert(pool, callback)
end

local function emit(index)
    pool[index]()
end

local function addCol(col)
    local width = col.width
    local caption = col.caption
    local Column = Feature.Columns.add()

    if width then
        Column.width = width
    end
    Column.caption = caption
end

local function addRow(row)
    local text = { row.hotkey, row.effect, row.description }
    local Item = Feature.Items.add()

    Item.caption = status.disable
    Item.SubItems.text = table.concat(text, '\n')

    addShortcut(Item, row.code, row.action)
end

local function draw()
    Prompt.width, Prompt.height = FEATURE_WIDTH, config.window.height
    Prompt.left = Cover.width
    Prompt.caption = "电波传达不到哦"
    Prompt.visible = false
    Prompt.bevelOuter = bvSpace

    Feature.width, Feature.height = FEATURE_WIDTH, config.window.height
    Feature.left = Cover.width
    Feature.borderStyle = bsNone
    Feature.readOnly = true
    Feature.rowSelect = true
    Feature.viewStyle = vsReport
    Feature.visible = false
    Feature.onClick = function(sender)
        local Item = sender.Selected

        if not Item then
            return
        end
        emit(sender.ItemIndex + 1)

        sender.Selected = nil
        sender.ItemIndex = -1
    end

    for _, col in ipairs(header) do
        addCol(col)
    end

    for _, row in pairs(keymap) do
        addRow(row)
    end
end

local function reset()
    for i = 0, Feature.Items.Count - 1 do
        Feature.Items[i].caption = status.disable
    end
end

local function show()
    Prompt.setVisible(false)
    Feature.setVisible(true)
end

local function hidden()
    Feature.setVisible(false)
    Prompt.setVisible(true)
    reset()
end

return {
    draw = draw,
    show = show,
    hidden = hidden,
}
