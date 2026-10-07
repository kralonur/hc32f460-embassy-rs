#[doc = "Register `OCMRHUH` reader"]
pub type R = crate::R<OcmrhuhSpec>;
#[doc = "Register `OCMRHUH` writer"]
pub type W = crate::W<OcmrhuhSpec>;
#[doc = "Field `OCFDCH` reader - desc OCFDCH"]
pub type OcfdchR = crate::BitReader;
#[doc = "Field `OCFDCH` writer - desc OCFDCH"]
pub type OcfdchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFPKH` reader - desc OCFPKH"]
pub type OcfpkhR = crate::BitReader;
#[doc = "Field `OCFPKH` writer - desc OCFPKH"]
pub type OcfpkhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFUCH` reader - desc OCFUCH"]
pub type OcfuchR = crate::BitReader;
#[doc = "Field `OCFUCH` writer - desc OCFUCH"]
pub type OcfuchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFZRH` reader - desc OCFZRH"]
pub type OcfzrhR = crate::BitReader;
#[doc = "Field `OCFZRH` writer - desc OCFZRH"]
pub type OcfzrhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPDCH` reader - desc OPDCH"]
pub type OpdchR = crate::FieldReader;
#[doc = "Field `OPDCH` writer - desc OPDCH"]
pub type OpdchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPPKH` reader - desc OPPKH"]
pub type OppkhR = crate::FieldReader;
#[doc = "Field `OPPKH` writer - desc OPPKH"]
pub type OppkhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPUCH` reader - desc OPUCH"]
pub type OpuchR = crate::FieldReader;
#[doc = "Field `OPUCH` writer - desc OPUCH"]
pub type OpuchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPZRH` reader - desc OPZRH"]
pub type OpzrhR = crate::FieldReader;
#[doc = "Field `OPZRH` writer - desc OPZRH"]
pub type OpzrhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPNPKH` reader - desc OPNPKH"]
pub type OpnpkhR = crate::FieldReader;
#[doc = "Field `OPNPKH` writer - desc OPNPKH"]
pub type OpnpkhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPNZRH` reader - desc OPNZRH"]
pub type OpnzrhR = crate::FieldReader;
#[doc = "Field `OPNZRH` writer - desc OPNZRH"]
pub type OpnzrhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc OCFDCH"]
    #[inline(always)]
    pub fn ocfdch(&self) -> OcfdchR {
        OcfdchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OCFPKH"]
    #[inline(always)]
    pub fn ocfpkh(&self) -> OcfpkhR {
        OcfpkhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OCFUCH"]
    #[inline(always)]
    pub fn ocfuch(&self) -> OcfuchR {
        OcfuchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OCFZRH"]
    #[inline(always)]
    pub fn ocfzrh(&self) -> OcfzrhR {
        OcfzrhR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc OPDCH"]
    #[inline(always)]
    pub fn opdch(&self) -> OpdchR {
        OpdchR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc OPPKH"]
    #[inline(always)]
    pub fn oppkh(&self) -> OppkhR {
        OppkhR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc OPUCH"]
    #[inline(always)]
    pub fn opuch(&self) -> OpuchR {
        OpuchR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc OPZRH"]
    #[inline(always)]
    pub fn opzrh(&self) -> OpzrhR {
        OpzrhR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc OPNPKH"]
    #[inline(always)]
    pub fn opnpkh(&self) -> OpnpkhR {
        OpnpkhR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc OPNZRH"]
    #[inline(always)]
    pub fn opnzrh(&self) -> OpnzrhR {
        OpnzrhR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc OCFDCH"]
    #[inline(always)]
    pub fn ocfdch(&mut self) -> OcfdchW<'_, OcmrhuhSpec> {
        OcfdchW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OCFPKH"]
    #[inline(always)]
    pub fn ocfpkh(&mut self) -> OcfpkhW<'_, OcmrhuhSpec> {
        OcfpkhW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OCFUCH"]
    #[inline(always)]
    pub fn ocfuch(&mut self) -> OcfuchW<'_, OcmrhuhSpec> {
        OcfuchW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OCFZRH"]
    #[inline(always)]
    pub fn ocfzrh(&mut self) -> OcfzrhW<'_, OcmrhuhSpec> {
        OcfzrhW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc OPDCH"]
    #[inline(always)]
    pub fn opdch(&mut self) -> OpdchW<'_, OcmrhuhSpec> {
        OpdchW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc OPPKH"]
    #[inline(always)]
    pub fn oppkh(&mut self) -> OppkhW<'_, OcmrhuhSpec> {
        OppkhW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc OPUCH"]
    #[inline(always)]
    pub fn opuch(&mut self) -> OpuchW<'_, OcmrhuhSpec> {
        OpuchW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc OPZRH"]
    #[inline(always)]
    pub fn opzrh(&mut self) -> OpzrhW<'_, OcmrhuhSpec> {
        OpzrhW::new(self, 10)
    }
    #[doc = "Bits 12:13 - desc OPNPKH"]
    #[inline(always)]
    pub fn opnpkh(&mut self) -> OpnpkhW<'_, OcmrhuhSpec> {
        OpnpkhW::new(self, 12)
    }
    #[doc = "Bits 14:15 - desc OPNZRH"]
    #[inline(always)]
    pub fn opnzrh(&mut self) -> OpnzrhW<'_, OcmrhuhSpec> {
        OpnzrhW::new(self, 14)
    }
}
#[doc = "desc OCMRHUH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrhuh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrhuh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmrhuhSpec;
impl crate::RegisterSpec for OcmrhuhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocmrhuh::R`](R) reader structure"]
impl crate::Readable for OcmrhuhSpec {}
#[doc = "`write(|w| ..)` method takes [`ocmrhuh::W`](W) writer structure"]
impl crate::Writable for OcmrhuhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCMRHUH to value 0"]
impl crate::Resettable for OcmrhuhSpec {}
