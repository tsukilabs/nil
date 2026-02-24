-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@class Client
client = {}

---@return ServerKind
function client:getServerKind() end

---@return string
function client:getServerVersion() end

---@return boolean
function client:isServerLocal() end

---@return boolean
function client:isServerReady() end

---@return boolean
function client:isServerRemote() end

---@return ServerAddr
function client:server() end

---@return string
function client:userAgent() end

---@return string
function client:version() end

---@return WorldId|nil
function client:world() end
