-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

---@meta

---@param req ForwardReportRequest
function client:forwardReport(req) end

---@param req GetReportRequest
---@return ReportKind
function client:getReport(req) end

---@param req GetReportsRequest
---@return ReportKind[]
function client:getReports(req) end
