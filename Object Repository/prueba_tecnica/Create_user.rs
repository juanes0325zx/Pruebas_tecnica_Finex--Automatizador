<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create_user</name>
   <tag></tag>
   <elementGuidId>8135906b-cec8-41b8-bdf0-5e0da0aac4aa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;active\&quot;: true,\n  \&quot;address\&quot;: ${GlobalVariable.address},\n  \&quot;city\&quot;: ${GlobalVariable.city},\n  \&quot;contract\&quot;: {\n    \&quot;contract\&quot;: {\n      \&quot;balance\&quot;: ${GlobalVariable.balance},\n      \&quot;beneficiary\&quot;: [\n        {\n          \&quot;id\&quot;: ${GlobalVariable.id_beneficiary},\n          \&quot;rate\&quot;: ${GlobalVariable.rare_beneficiary}\n        }\n      ],\n      \&quot;beneficiaryState\&quot;: {\n        \&quot;id\&quot;: ${GlobalVariable.id_beneficiaryState},\n        \&quot;name\&quot;: ${GlobalVariable.name_beneficiaryState}\n      },\n      \&quot;contractPermanence\&quot;: {\n        \&quot;contract\&quot;: ${GlobalVariable.contract_contractPermanence},\n        \&quot;endPermanencePeriod\&quot;: ${GlobalVariable.endPermanencePeriod_contractPermanence},\n        \&quot;id\&quot;: ${GlobalVariable.id_contractPermanence},\n        \&quot;months\&quot;: ${GlobalVariable.months_contractPermanence},\n        \&quot;permanence\&quot;: ${GlobalVariable.permanence_contractPermanence}\n      },\n      \&quot;creator\&quot;: ${GlobalVariable.creator_contractPermanence},\n      \&quot;dailyRate\&quot;: ${GlobalVariable.dailyRate_contractPermanence},\n      \&quot;documentationSupport\&quot;: ${GlobalVariable.documentationSupport_contractPermanence},\n      \&quot;finishDate\&quot;: ${GlobalVariable.finishDate_contractPermanence},\n      \&quot;id\&quot;: ${GlobalVariable.id_contractPermanence},\n      \&quot;initialBalance\&quot;: ${GlobalVariable.initialBalance_contractPermanence},\n      \&quot;lastModifiedBy\&quot;: ${GlobalVariable.lastModifiedBy_contractPermanence},\n      \&quot;paymentType\&quot;: {\n        \&quot;id\&quot;: ${GlobalVariable.id_paymentType},\n        \&quot;name\&quot;: ${GlobalVariable.name_paymentType}\n      },\n      \&quot;startDate\&quot;: ${GlobalVariable.startDate_paymentType},\n      \&quot;status\&quot;: {\n        \&quot;id\&quot;: ${GlobalVariable.id_status},\n        \&quot;name\&quot;: ${GlobalVariable.name_status}\n      },\n      \&quot;titularRate\&quot;: ${GlobalVariable.titularRate},\n      \&quot;type\&quot;: {\n        \&quot;id\&quot;: ${GlobalVariable.id_type},\n        \&quot;name\&quot;: ${GlobalVariable.name_type}\n      }\n    },\n    \&quot;id\&quot;: ${GlobalVariable.id_paymentType}\n  },\n  \&quot;country\&quot;: ${GlobalVariable.country},\n  \&quot;creationDate\&quot;: ${GlobalVariable.creationDate},\n  \&quot;creator\&quot;: ${GlobalVariable.creator},\n  \&quot;dailyEmail\&quot;: ${GlobalVariable.dailyEmail},\n  \&quot;documentNumber\&quot;: ${GlobalVariable.documentNumber},\n  \&quot;documentType\&quot;: {\n    \&quot;abbreviation\&quot;: ${GlobalVariable.abbreviation_documentType},\n    \&quot;id\&quot;: ${GlobalVariable.id_documentType},\n    \&quot;name\&quot;: ${GlobalVariable.name_documentType}\n  },\n  \&quot;documentationStatus\&quot;: true,\n  \&quot;documentationSupport\&quot;: ${GlobalVariable.documentationSupport},\n  \&quot;email\&quot;: ${GlobalVariable.email},\n  \&quot;firstLastName\&quot;: ${GlobalVariable.firstLastName},\n  \&quot;firstName\&quot;: ${GlobalVariable.firstName},\n  \&quot;id\&quot;: ${GlobalVariable.id},\n  \&quot;indicative\&quot;: ${GlobalVariable.indicative},\n  \&quot;lastModifiedBy\&quot;: ${GlobalVariable.lastModifiedBy},\n  \&quot;mobile\&quot;: ${GlobalVariable.mobile},\n  \&quot;phone\&quot;: ${GlobalVariable.address},\n  \&quot;secondLastName\&quot;: ${GlobalVariable.secondLastName},\n  \&quot;secondName\&quot;: ${GlobalVariable.secondName},\n  \&quot;state\&quot;: ${GlobalVariable.state},\n  \&quot;userLogin\&quot;: {\n    \&quot;attempts\&quot;: ${GlobalVariable.attempts_userLogin},\n    \&quot;changePasswordDate\&quot;: ${GlobalVariable.changePassWordDate_userLogin},\n    \&quot;changePasswordForce\&quot;: true,\n    \&quot;id\&quot;: ${GlobalVariable.id_userLogin},\n    \&quot;password\&quot;: ${GlobalVariable.password_userLogin},\n    \&quot;sessionId\&quot;: ${GlobalVariable.sessionId_userLogin},\n    \&quot;status\&quot;: ${GlobalVariable.status_userLogin}\n  },\n  \&quot;userType\&quot;: {\n    \&quot;id\&quot;: ${GlobalVariable.id_userType},\n    \&quot;name\&quot;: ${GlobalVariable.name_userType}\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Connection</name>
      <type>Main</type>
      <value>keep-alive</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Encoding</name>
      <type>Main</type>
      <value>gzip, deflate, br</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://unit-test.midaily.co/api/v1/user/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
