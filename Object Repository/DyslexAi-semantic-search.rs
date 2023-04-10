<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DyslexAi-semantic-search</name>
   <tag></tag>
   <elementGuidId>496c7bc0-d309-4f02-8969-4c2e2c022303</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;query\&quot;: \&quot;Hoe veel weerberichten?\&quot;,\n\&quot;text\&quot;: [\&quot;Deboosere maakt zijn debuut op 12 maart 1987, bijna dag op dag 36 jaar voor zijn allerlaatste weerpraatje.\&quot;, \&quot;Gedurende zijn hele carrière heeft hij ongeveer 100 000 (!) weerberichten gepresenteerd.\&quot;, \&quot;\\\&quot;Dat aantal: ik kon het zelf ook niet geloven, maar het is echt in die orde.\&quot;, \&quot;De wolken zijn mijn vrienden, de zon is mijn vriend, de blauwe hemel, de sterren.\&quot;, \&quot;Anders hou je ook niet zoveel jaren vol.\&quot;, \&quot;Het weer stopt ook nooit.\&quot;, \&quot;Zelfs God kon zeggen: \\\&quot;De zevende dag rust ik\\\&quot;.\&quot;, \&quot;Maar dat geldt niet voor weermannen en -vrouwen.\\\&quot;\&quot;]\n}&quot;,
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
      <webElementGuid>b40b3100-c022-4159-ba0e-9946a4d7b684</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dyslexai.designproject.idlab.ugent.be/dev/dyslexai/search</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
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
