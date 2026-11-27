--
-- PostgreSQL database dump
--

\restrict UrPM0XhIBmKXrjrwJruJCZmHQVpGcsnY9hTi1StkiFJZGowu7LnzrAwwJuf166L

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character varying(2),
    pduration character(20) NOT NULL,
    project_manegerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_manegerid) FROM stdin;
13	A	9 months            	102
22	B	14 months           	97
33	C	16 months           	120
44	D	25 months           	108
55	E	9 months            	107
\.


--
-- PostgreSQL database dump complete
--

\unrestrict UrPM0XhIBmKXrjrwJruJCZmHQVpGcsnY9hTi1StkiFJZGowu7LnzrAwwJuf166L

