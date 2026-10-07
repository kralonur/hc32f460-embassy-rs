#[doc = "Register `OCMRLVL` reader"]
pub type R = crate::R<OcmrlvlSpec>;
#[doc = "Register `OCMRLVL` writer"]
pub type W = crate::W<OcmrlvlSpec>;
#[doc = "Field `OCFDCL` reader - desc OCFDCL"]
pub type OcfdclR = crate::BitReader;
#[doc = "Field `OCFDCL` writer - desc OCFDCL"]
pub type OcfdclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFPKL` reader - desc OCFPKL"]
pub type OcfpklR = crate::BitReader;
#[doc = "Field `OCFPKL` writer - desc OCFPKL"]
pub type OcfpklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFUCL` reader - desc OCFUCL"]
pub type OcfuclR = crate::BitReader;
#[doc = "Field `OCFUCL` writer - desc OCFUCL"]
pub type OcfuclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFZRL` reader - desc OCFZRL"]
pub type OcfzrlR = crate::BitReader;
#[doc = "Field `OCFZRL` writer - desc OCFZRL"]
pub type OcfzrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPDCL` reader - desc OPDCL"]
pub type OpdclR = crate::FieldReader;
#[doc = "Field `OPDCL` writer - desc OPDCL"]
pub type OpdclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPPKL` reader - desc OPPKL"]
pub type OppklR = crate::FieldReader;
#[doc = "Field `OPPKL` writer - desc OPPKL"]
pub type OppklW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPUCL` reader - desc OPUCL"]
pub type OpuclR = crate::FieldReader;
#[doc = "Field `OPUCL` writer - desc OPUCL"]
pub type OpuclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPZRL` reader - desc OPZRL"]
pub type OpzrlR = crate::FieldReader;
#[doc = "Field `OPZRL` writer - desc OPZRL"]
pub type OpzrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPNPKL` reader - desc OPNPKL"]
pub type OpnpklR = crate::FieldReader;
#[doc = "Field `OPNPKL` writer - desc OPNPKL"]
pub type OpnpklW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPNZRL` reader - desc OPNZRL"]
pub type OpnzrlR = crate::FieldReader;
#[doc = "Field `OPNZRL` writer - desc OPNZRL"]
pub type OpnzrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPNDCL` reader - desc EOPNDCL"]
pub type EopndclR = crate::FieldReader;
#[doc = "Field `EOPNDCL` writer - desc EOPNDCL"]
pub type EopndclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPNUCL` reader - desc EOPNUCL"]
pub type EopnuclR = crate::FieldReader;
#[doc = "Field `EOPNUCL` writer - desc EOPNUCL"]
pub type EopnuclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPDCL` reader - desc EOPDCL"]
pub type EopdclR = crate::FieldReader;
#[doc = "Field `EOPDCL` writer - desc EOPDCL"]
pub type EopdclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPPKL` reader - desc EOPPKL"]
pub type EoppklR = crate::FieldReader;
#[doc = "Field `EOPPKL` writer - desc EOPPKL"]
pub type EoppklW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPUCL` reader - desc EOPUCL"]
pub type EopuclR = crate::FieldReader;
#[doc = "Field `EOPUCL` writer - desc EOPUCL"]
pub type EopuclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPZRL` reader - desc EOPZRL"]
pub type EopzrlR = crate::FieldReader;
#[doc = "Field `EOPZRL` writer - desc EOPZRL"]
pub type EopzrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPNPKL` reader - desc EOPNPKL"]
pub type EopnpklR = crate::FieldReader;
#[doc = "Field `EOPNPKL` writer - desc EOPNPKL"]
pub type EopnpklW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPNZRL` reader - desc EOPNZRL"]
pub type EopnzrlR = crate::FieldReader;
#[doc = "Field `EOPNZRL` writer - desc EOPNZRL"]
pub type EopnzrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc OCFDCL"]
    #[inline(always)]
    pub fn ocfdcl(&self) -> OcfdclR {
        OcfdclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OCFPKL"]
    #[inline(always)]
    pub fn ocfpkl(&self) -> OcfpklR {
        OcfpklR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OCFUCL"]
    #[inline(always)]
    pub fn ocfucl(&self) -> OcfuclR {
        OcfuclR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OCFZRL"]
    #[inline(always)]
    pub fn ocfzrl(&self) -> OcfzrlR {
        OcfzrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc OPDCL"]
    #[inline(always)]
    pub fn opdcl(&self) -> OpdclR {
        OpdclR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc OPPKL"]
    #[inline(always)]
    pub fn oppkl(&self) -> OppklR {
        OppklR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc OPUCL"]
    #[inline(always)]
    pub fn opucl(&self) -> OpuclR {
        OpuclR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc OPZRL"]
    #[inline(always)]
    pub fn opzrl(&self) -> OpzrlR {
        OpzrlR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc OPNPKL"]
    #[inline(always)]
    pub fn opnpkl(&self) -> OpnpklR {
        OpnpklR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc OPNZRL"]
    #[inline(always)]
    pub fn opnzrl(&self) -> OpnzrlR {
        OpnzrlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc EOPNDCL"]
    #[inline(always)]
    pub fn eopndcl(&self) -> EopndclR {
        EopndclR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - desc EOPNUCL"]
    #[inline(always)]
    pub fn eopnucl(&self) -> EopnuclR {
        EopnuclR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc EOPDCL"]
    #[inline(always)]
    pub fn eopdcl(&self) -> EopdclR {
        EopdclR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc EOPPKL"]
    #[inline(always)]
    pub fn eoppkl(&self) -> EoppklR {
        EoppklR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - desc EOPUCL"]
    #[inline(always)]
    pub fn eopucl(&self) -> EopuclR {
        EopuclR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - desc EOPZRL"]
    #[inline(always)]
    pub fn eopzrl(&self) -> EopzrlR {
        EopzrlR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - desc EOPNPKL"]
    #[inline(always)]
    pub fn eopnpkl(&self) -> EopnpklR {
        EopnpklR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - desc EOPNZRL"]
    #[inline(always)]
    pub fn eopnzrl(&self) -> EopnzrlR {
        EopnzrlR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc OCFDCL"]
    #[inline(always)]
    pub fn ocfdcl(&mut self) -> OcfdclW<'_, OcmrlvlSpec> {
        OcfdclW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OCFPKL"]
    #[inline(always)]
    pub fn ocfpkl(&mut self) -> OcfpklW<'_, OcmrlvlSpec> {
        OcfpklW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OCFUCL"]
    #[inline(always)]
    pub fn ocfucl(&mut self) -> OcfuclW<'_, OcmrlvlSpec> {
        OcfuclW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OCFZRL"]
    #[inline(always)]
    pub fn ocfzrl(&mut self) -> OcfzrlW<'_, OcmrlvlSpec> {
        OcfzrlW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc OPDCL"]
    #[inline(always)]
    pub fn opdcl(&mut self) -> OpdclW<'_, OcmrlvlSpec> {
        OpdclW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc OPPKL"]
    #[inline(always)]
    pub fn oppkl(&mut self) -> OppklW<'_, OcmrlvlSpec> {
        OppklW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc OPUCL"]
    #[inline(always)]
    pub fn opucl(&mut self) -> OpuclW<'_, OcmrlvlSpec> {
        OpuclW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc OPZRL"]
    #[inline(always)]
    pub fn opzrl(&mut self) -> OpzrlW<'_, OcmrlvlSpec> {
        OpzrlW::new(self, 10)
    }
    #[doc = "Bits 12:13 - desc OPNPKL"]
    #[inline(always)]
    pub fn opnpkl(&mut self) -> OpnpklW<'_, OcmrlvlSpec> {
        OpnpklW::new(self, 12)
    }
    #[doc = "Bits 14:15 - desc OPNZRL"]
    #[inline(always)]
    pub fn opnzrl(&mut self) -> OpnzrlW<'_, OcmrlvlSpec> {
        OpnzrlW::new(self, 14)
    }
    #[doc = "Bits 16:17 - desc EOPNDCL"]
    #[inline(always)]
    pub fn eopndcl(&mut self) -> EopndclW<'_, OcmrlvlSpec> {
        EopndclW::new(self, 16)
    }
    #[doc = "Bits 18:19 - desc EOPNUCL"]
    #[inline(always)]
    pub fn eopnucl(&mut self) -> EopnuclW<'_, OcmrlvlSpec> {
        EopnuclW::new(self, 18)
    }
    #[doc = "Bits 20:21 - desc EOPDCL"]
    #[inline(always)]
    pub fn eopdcl(&mut self) -> EopdclW<'_, OcmrlvlSpec> {
        EopdclW::new(self, 20)
    }
    #[doc = "Bits 22:23 - desc EOPPKL"]
    #[inline(always)]
    pub fn eoppkl(&mut self) -> EoppklW<'_, OcmrlvlSpec> {
        EoppklW::new(self, 22)
    }
    #[doc = "Bits 24:25 - desc EOPUCL"]
    #[inline(always)]
    pub fn eopucl(&mut self) -> EopuclW<'_, OcmrlvlSpec> {
        EopuclW::new(self, 24)
    }
    #[doc = "Bits 26:27 - desc EOPZRL"]
    #[inline(always)]
    pub fn eopzrl(&mut self) -> EopzrlW<'_, OcmrlvlSpec> {
        EopzrlW::new(self, 26)
    }
    #[doc = "Bits 28:29 - desc EOPNPKL"]
    #[inline(always)]
    pub fn eopnpkl(&mut self) -> EopnpklW<'_, OcmrlvlSpec> {
        EopnpklW::new(self, 28)
    }
    #[doc = "Bits 30:31 - desc EOPNZRL"]
    #[inline(always)]
    pub fn eopnzrl(&mut self) -> EopnzrlW<'_, OcmrlvlSpec> {
        EopnzrlW::new(self, 30)
    }
}
#[doc = "desc OCMRLVL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrlvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrlvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmrlvlSpec;
impl crate::RegisterSpec for OcmrlvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocmrlvl::R`](R) reader structure"]
impl crate::Readable for OcmrlvlSpec {}
#[doc = "`write(|w| ..)` method takes [`ocmrlvl::W`](W) writer structure"]
impl crate::Writable for OcmrlvlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCMRLVL to value 0"]
impl crate::Resettable for OcmrlvlSpec {}
