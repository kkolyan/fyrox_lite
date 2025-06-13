---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_input::Input
-----------------------------------------------------------

--- <p>Utility class to poll player input events</p>
---@class Input_static
---@field MouseLeft number ---<p>Left Mouse Button ID</p>
---@field MouseRight number ---
---@field MouseMiddle number ---
---@field MouseBack number ---
---@field MouseForward number ---
---@field mouse_move Vector2 
---@field mouse_scroll Vector2 
Input = {}

---@class Input
Input_instance = {}

--- <p>is mouse button with ID <code>button</code> is in pressed state?</p>
---@param button number
---@return boolean
function Input:is_mouse_button_down(button) end

---@param button number
---@return boolean
function Input:is_mouse_button_up(button) end

---@param button number
---@return boolean
function Input:is_mouse_button_pressed(button) end

---@param key KeyCode
---@return boolean
function Input:is_key_down(key) end

---@param key KeyCode
---@return boolean
function Input:is_key_up(key) end

---@param key KeyCode
---@return boolean
function Input:is_key_pressed(key) end

