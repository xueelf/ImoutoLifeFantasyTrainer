local util = require('util')
local app = require('config.app')
local keymap = require('config.keymap')
local pointer = require('config.pointer')

local MainForm = getMainForm()
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

local function createObserver()
    local subscriptions = {}
    local timer = createTimer(MainForm)

    timer.setInterval(1000)
    timer.setOnTimer(function()
        for _, subscription in ipairs(subscriptions) do
            util.setPointerValue(subscription.offsets, subscription.value or subscription.default)
        end
    end)

    return {
        observe = function(offsets, value)
            table.insert(subscriptions, {
                offsets = offsets,
                value = value,
                default = util.getPointerValue(offsets),
            })
        end,
        unobserve = function(offsets)
            for index, subscription in ipairs(subscriptions) do
                if subscription.offsets == offsets then
                    table.remove(subscriptions, index)
                    util.setPointerValue(subscription.offsets, subscription.default)

                    return
                end
            end
        end,
        disconnect = function()
            subscriptions = {}
        end,
    }
end

local observer = createObserver()

local handler = {
    speed = function(enabled)
        if enabled then
            speedhack_setSpeed(3)
        else
            speedhack_setSpeed(1)
        end
        MainForm.cbSpeedhack.checked = enabled
    end,
    winer = function(enabled)
        if enabled then
            observer.observe(pointer.battle.winner, 1)
        else
            observer.unobserve(pointer.battle.winner)
        end
    end,
    chronostasis = function(enabled)
        if enabled then
            observer.observe(pointer.status.time.hour)
            observer.observe(pointer.status.time.minute)
            observer.observe(pointer.status.time.week)
        else
            observer.unobserve(pointer.status.time.hour)
            observer.unobserve(pointer.status.time.minute)
            observer.unobserve(pointer.status.time.week)
        end
    end,
    ablutomania = function(enabled)
        if enabled then
            observer.observe(pointer.status.took_bath, 0)
        else
            observer.unobserve(pointer.status.took_bath)
        end
    end,
    teaaholic = function(enabled)
        if enabled then
            observer.observe(pointer.status.had_tea, 0)
        else
            observer.unobserve(pointer.status.had_tea)
        end
    end,
}
local pool = {}

local function addShortcut(owner, code, action)
    if not owner then
        return
    end
    local callback = function()
        local method = handler[action]

        if not method then
            showMessage(string.format('Action "%s" is not defined', action))
            return
        end
        local enabled = owner.caption == status.disable
        local success, result = pcall(method, enabled)

        if not success then
            showMessage(result)
            return
        end

        if enabled then
            owner.setCaption(status.enable)
            util.playVoice('Select')
        else
            owner.setCaption(status.disable)
            util.playVoice('Cancel')
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
    Prompt.width, Prompt.height = FEATURE_WIDTH, app.window.height
    Prompt.left = Cover.width
    Prompt.caption = '电波传达不到哦'
    Prompt.visible = false
    Prompt.bevelOuter = bvSpace

    Feature.width, Feature.height = FEATURE_WIDTH, app.window.height
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

local function resetStatus()
    for i = 0, Feature.Items.Count - 1 do
        Feature.Items[i].caption = status.disable
    end
    observer.disconnect()
end

local function attach()
    Prompt.setVisible(false)
    Feature.setVisible(true)
end

local function detach()
    Feature.setVisible(false)
    Prompt.setVisible(true)
    resetStatus()
end

return {
    draw = draw,
    attach = attach,
    detach = detach,
}
